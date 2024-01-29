pub mod url {

    const URL_REGEX: String = "slkfdjslf".parse().unwrap();

    struct URLPool {
        pub url_list: Vec<String>,
        pub axioms: Vec<String>,
        pub rows: u16,
        pub columns: u16,
    }

    pub fn obfuscate_url (rows: u16, columns:u16) {
        println!("The value of x is:  {rows} {columns} and regex {URL_REGEX}");
    }

    //fn remath_matrix_2d<T>(_s: SGen<T>) {}
}