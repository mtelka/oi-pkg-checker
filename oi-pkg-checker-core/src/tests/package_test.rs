use fmri::FMRI;

use crate::packages::components::Components;
use crate::packages::depend_types::DependTypes;
use crate::packages::dependencies::Dependencies;
use crate::packages::dependency::Dependency;
use crate::packages::package::Package;

#[test]
fn is_fmri_needed_as_dependency() {
    let mut package = Package::new(
        FMRI::parse_raw(&"pkg:/test@2.3.2".to_owned()).unwrap(),
        false,
        false,
    );
    let mut dependencies = Dependencies::new();
    dependencies.add(Dependency::new(&mut DependTypes::Require(
        FMRI::parse_raw(&"pkg:/audio/audacity@2.3.2-2022.0.0.1".to_owned()).unwrap(),
    )));
    dependencies.add(Dependency::new(&mut DependTypes::Require(
        FMRI::parse_raw(&"pkg:/library/libvorbis@1.3.7-2022.0.0.0".to_owned()).unwrap(),
    )));
    package.add_runtime_dependencies(dependencies);

    assert_eq!(
        package
            .is_fmri_needed_as_dependency(
                &Components::new(),
                &FMRI::parse_raw(
                    &"pkg:/audio/audacity@2.3.2,5.11-2022.0.0.1:20220126T070330Z".to_owned()
                )
                .unwrap()
            )
            .is_some(),
        true
    );

    assert_eq!(
        package
            .is_fmri_needed_as_dependency(
                &Components::new(),
                &FMRI::parse_raw(
                    &"pkg:/audio/audacity@3.3.2,5.11-2022.0.0.1:20220126T070330Z".to_owned()
                )
                .unwrap()
            )
            .is_some(),
        true
    );

    assert_eq!(
        package
            .is_fmri_needed_as_dependency(
                &Components::new(),
                &FMRI::parse_raw(
                    &"pkg:/audio/audacity@1.3.2,5.11-2022.0.0.1:20220126T070330Z".to_owned()
                )
                .unwrap()
            )
            .is_some(),
        false
    );

    assert_eq!(
        package
            .is_fmri_needed_as_dependency(
                &Components::new(),
                &FMRI::parse_raw(
                    &"pkg:/library/libvorbis@1.3.7,1-2022.0.0.0:20220126T070330Z".to_owned()
                )
                .unwrap()
            )
            .is_some(),
        true
    );

    assert_eq!(
        package
            .is_fmri_needed_as_dependency(
                &Components::new(),
                &FMRI::parse_raw(
                    &"pkg:/library/libvorbis@2.3.7,1-2022.0.0.0:20220126T070330Z".to_owned()
                )
                .unwrap()
            )
            .is_some(),
        true
    );

    assert_eq!(
        package
            .is_fmri_needed_as_dependency(
                &Components::new(),
                &FMRI::parse_raw(
                    &"pkg:/library/libvorbis@1.2.7,1-2022.0.0.0:20220126T070330Z".to_owned()
                )
                .unwrap()
            )
            .is_some(),
        false
    );

    assert_eq!(
        package
            .is_fmri_needed_as_dependency(
                &Components::new(),
                &FMRI::parse_raw(&"pkg:/test@2.54.2".to_owned()).unwrap()
            )
            .is_some(),
        false
    );

    assert_eq!(
        package
            .is_fmri_needed_as_dependency(
                &Components::new(),
                &FMRI::parse_raw(&"pkg:/test@1.3.2".to_owned()).unwrap()
            )
            .is_some(),
        false
    );
}
