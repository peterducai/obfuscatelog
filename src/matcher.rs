use regex::Regex;

pub mod matcher {

//     excludedIPs = map[string]struct{}{
// "127.0.0.1": {},
// "0.0.0.0":   {},
// "::1":       {},


    const DOMAIN: &str = "([a-zA-Z0-9\\.]*\\.)?(%s)";
    const IPV4: &str = "(([1-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])[.]([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])[.]([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])[.]|([1-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])[_-]([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])[_-]([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])[_-])([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5]){1,3}";
    const IPV6: &str = "(([a-f0-9]{0,4}[:]){1,8}([0-9a-fA-F]{1,4}|::)+)";

    // this regex differs from the standard `(?:[0-9a-fA-F]([:-])?){12}`, to not match very frequently happening UUIDs in K8s
    // the main culprit is the support for squashed MACs like '69806FE67C05', which won't be supported with the below
    const MAC: &str = "([0-9a-fA-F]{2}[:-]){5}[0-9a-fA-F]{2}";
    const EMAIL: &str = "[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}";


    pub fn match_word(line: &str, wrd: &str) {
        println!("wordmatch:");
        let word = "\\b".to_string()+wrd+"\\b";
        // println!("matching {}", word);
        let re = regex::Regex::new(&word).unwrap();
        for cap in re.captures_iter(&line) {
            println!("{}", cap.get(0).unwrap().as_str());
        }
    }

    pub fn match_ipv4(line: String) {
        let re = regex::Regex::new(IPV4).unwrap();
        for cap in re.captures_iter(&line) {
            println!("{}", cap.get(0).unwrap().as_str());
        }
    }

    pub fn match_ipv6(line: String) {
        let re = regex::Regex::new(IPV6).unwrap();
        for cap in re.captures_iter(&line) {
            println!("{}", cap.get(0).unwrap().as_str());
        }
    }
}