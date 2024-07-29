use crate::{
    packages::{
        dependency_type::{
            DependencyTypes,
            DependencyTypes::{Build, Runtime, SystemBuild, SystemTest, Test},
        },
        rev_depend_type::RevDependType,
    },
    problems::{Problem, Problem::PackageInMultipleComponents},
    Component, DependTypes,
};
use fmri::{Version, FMRI};
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, cmp::Ordering, rc::Rc};

/// Package. Can hold multiple versions with different runtime dependencies.
#[derive(Clone, Debug)]
pub struct Package {
    /// contains no version
    pub(crate) fmri: FMRI,
    /// versions of package
    pub(crate) versions: Vec<PackageVersion>,
    /// reference to component, if package is in component
    pub(crate) component: Option<Rc<RefCell<Component>>>,
    pub(crate) obsolete: bool,
    pub(crate) renamed: bool,
    /// packages that depend on this package
    pub(crate) runtime_dependents: Vec<RevDependType>,
    pub(crate) build_dependents: Vec<Rc<RefCell<Component>>>,
    pub(crate) test_dependents: Vec<Rc<RefCell<Component>>>,
    pub(crate) sys_build_dependents: Vec<Rc<RefCell<Component>>>,
    pub(crate) sys_test_dependents: Vec<Rc<RefCell<Component>>>,
}

impl Package {
    /// for creating new empty package
    pub fn new(fmri: FMRI) -> Self {
        Self {
            fmri,
            versions: Vec::new(),
            component: None,
            runtime_dependents: Vec::new(),
            obsolete: false,
            renamed: false,
            build_dependents: Vec::new(),
            test_dependents: Vec::new(),
            sys_build_dependents: Vec::new(),
            sys_test_dependents: Vec::new(),
        }
    }

    pub fn add_package_version(&mut self, package_version: PackageVersion) -> Result<(), String> {
        if self.versions.contains(&package_version) {
            return Ok(());
        }

        for ver in &self.versions {
            if ver.version.cmp(&package_version.version) == Ordering::Equal {
                return Ok(());
            }
        }

        if package_version.is_obsolete() && package_version.is_renamed() {
            return Err(format!(
                "package cannot be obsolete and renamed at the same time, package: {:?}",
                package_version
            ));
        }

        self.set_obsolete(package_version.is_obsolete());
        self.set_renamed(package_version.is_renamed());

        self.versions.push(package_version);
        Ok(())
    }

    pub fn add_dependent(
        &mut self,
        dependent: Rc<RefCell<Component>>,
        dependency_type: &DependencyTypes,
    ) -> Result<(), String> {
        match dependency_type {
            Runtime => return Err("you can not add runtime dependent".to_owned()),
            Build => self.build_dependents.push(dependent),
            Test => self.test_dependents.push(dependent),
            SystemBuild => self.sys_build_dependents.push(dependent),
            SystemTest => self.sys_test_dependents.push(dependent),
        }
        Ok(())
    }

    pub fn set_component(&mut self, component: Rc<RefCell<Component>>) -> Option<Box<Problem>> {
        if let Some(c) = &self.component {
            return Some(Box::new(PackageInMultipleComponents(
                self.fmri.clone(),
                vec![
                    c.borrow().get_name().to_owned(),
                    component.borrow().get_name().to_owned(),
                ],
            )));
        } else {
            self.component = Some(component)
        }
        None
    }

    pub fn get_versions(&mut self) -> &mut Vec<PackageVersion> {
        &mut self.versions
    }

    pub fn set_obsolete(&mut self, obsolete: bool) {
        self.obsolete = obsolete
    }

    pub fn set_renamed(&mut self, renamed: bool) {
        self.renamed = renamed
    }

    pub fn is_obsolete(&self) -> bool {
        self.obsolete
    }

    pub fn is_renamed(&self) -> bool {
        self.renamed
    }

    pub fn is_in_component(&self) -> &Option<Rc<RefCell<Component>>> {
        &self.component
    }

    pub fn get_runtime_dependents(&self) -> &Vec<RevDependType> {
        &self.runtime_dependents
    }

    pub fn get_git_dependents(
        &self,
        dependency_type: DependencyTypes,
    ) -> Result<&Vec<Rc<RefCell<Component>>>, String> {
        Ok(match dependency_type {
            Runtime => return Err("you can not add runtime dependent".to_owned()),
            Build => &self.build_dependents,
            Test => &self.test_dependents,
            SystemBuild => &self.sys_build_dependents,
            SystemTest => &self.sys_test_dependents,
        })
    }

    pub fn change_versions(&mut self, vers: Vec<PackageVersion>) {
        self.versions = vers
    }
}

/// PackageVersion represents one version of package
#[derive(Serialize, Deserialize, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PackageVersion {
    /// package version
    pub(crate) version: Version,
    /// runtime dependencies
    pub(crate) runtime: Vec<DependTypes>,
    obsolete: bool,
    renamed: bool,
}

impl PackageVersion {
    /// for creating new empty version of package
    pub fn new(version: Version) -> Self {
        Self {
            version,
            runtime: vec![],
            obsolete: false,
            renamed: false,
        }
    }

    /// `package` argument points to the pacakge where this `package version` belongs
    pub fn add_runtime_dependencies(&mut self, runtime: &mut Vec<DependTypes>) -> &Self {
        self.runtime.append(runtime);
        self
    }

    pub fn set_obsolete(&mut self, obsolete: bool) -> &Self {
        self.obsolete = obsolete;
        self
    }

    pub fn set_renamed(&mut self, renamed: bool) -> &Self {
        self.renamed = renamed;
        self
    }

    pub fn is_obsolete(&self) -> bool {
        self.obsolete
    }

    pub fn is_renamed(&self) -> bool {
        self.renamed
    }
}
