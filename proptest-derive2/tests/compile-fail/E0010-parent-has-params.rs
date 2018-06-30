#[macro_use]
extern crate proptest_derive;

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0010]
#[proptest(no_params)]
struct T0 {
    #[proptest(no_params)]
    field: String
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0010]
#[proptest(no_params)]
struct T1(
    #[proptest(no_params)]
    String
);

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0010]
#[proptest(params = "u8")]
struct T2 {
    #[proptest(no_params)]
    bar: String
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0010]
#[proptest(params = "usize")]
struct T3(
    #[proptest(no_params)]
    String
);

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0010]
#[proptest(no_params)]
struct T4 {
    #[proptest(params = "usize")]
    baz: String
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0010]
#[proptest(no_params)]
struct T5(
    #[proptest(params = "String")]
    String
);

// TODO: THIS IS A PROBLEM!
#[derive(Debug, Arbitrary)] // ERROR: [proptest_derive, E0010]
#[proptest(no_params)]
enum T6 {
    #[proptest(params = "String")]
    V0(u8),
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0010]
enum T7 {
    #[proptest(no_params)]
    V0(
        #[proptest(params = "String")]
        u8
    ),
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0010]
#[proptest(no_params)]
enum T8 {
    V0(
        #[proptest(params = "String")]
        u8
    ),
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0010]
#[proptest(params = "String")]
enum T9 {
    V0(
        #[proptest(no_params)]
        u8
    ),
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0010]
#[proptest(params = "String")]
enum T10 {
    V0 {
        #[proptest(no_params)]
        batman: u8
    },
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0010]
#[proptest(no_params)]
enum T11 {
    V0 {
        #[proptest(params = "String")]
        batman: u8
    },
}
