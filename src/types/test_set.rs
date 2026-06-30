
use bevy_ecs::schedule::SystemSet;

/// Used to tell Bevy on .add_systems() calls which tests to run first.  When used in conjunction
/// with .chain().in_set(INSERT_ENUM_VALUE_HERE) on .add_systems() and with a followed .configure_sets()
/// call, it will force systems to run in sequential order even when they are split by different
/// .add_systems() calls. The reason why this exists is to get around Bevy's concurrent running of
/// add_system groups.
///
/// EXAMPLE
/// ```ignore
/// .add_systems(Startup, (
///     system_1,
///     system_2,
/// ).chain().in_set(TestSet::First))
///
/// .add_systems(Startup, (
///     system_3,
///     system_4,
/// ).chain().in_set(TestSet::Second))
///
/// .configure_sets(Startup, TestSet::First.before(TestSet::Second))
/// ```
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum TestSet {
    Set0,
    Set1,
    Set2,
    Set3,
    Set4,
    Set5,
    Set6,
    Set7,
    Set8,
    Set9,
}
