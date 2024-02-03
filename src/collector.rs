pub mod collector {

    pub struct Collect {
        domains: Vec<String>,
        ipv4s: Vec<String>,
        ipv6s: Vec<String>,
        macs: Vec<String>,
        emails: Vec<String>,
    }

    impl Collect {
        pub fn new() -> Collect {
            Collect {
                domains: Vec::new(),
                ipv4s: Vec::new(),
                ipv6s: Vec::new(),
                macs: Vec::new(),
                emails: Vec::new(),
            }
        }

        pub fn push_domain(&mut self, s: String) {
            self.domains.push(s);
        }

        pub fn push_ipv4(&mut self, s: String) {
            self.ipv4s.push(s);
        }

        pub fn push_ipv6(&mut self, s: String) {
            self.ipv6s.push(s);
        }

        pub fn push_mac(&mut self, s: String) {
            self.macs.push(s);
        }

        pub fn push_email(&mut self, s: String) {
            self.emails.push(s);
        }
    }

}