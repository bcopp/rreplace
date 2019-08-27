
use std::collections::HashMap;
use std::collections::LinkedList;

    
pub mod Replacer{
    use super::*;

    pub fn run<'a>(from: &str, seqs_hash: HashMap<&'a str, &'a str>) -> String{
        let mut all_matches: LinkedList<Matcher> = LinkedList::new();
        let seqs = hash_to_vec_ordered_longest_shortest(seqs_hash);
        let mut to_str = String::from("");
        let mut memory = StringCache::new();

        for (c_index, c) in from.char_indices(){
            // Build new matches
            if let Some(matches) = build_matches(c_index, c, &seqs){
                for matcher in matches{
                    all_matches.push_back(matcher);
                }
            };

            // If matches exist
            if ! all_matches.is_empty(){

                // Run and keep the Completed and running
                for matcher in &mut all_matches{
                    matcher.run(c);
                }
                all_matches = all_matches.into_iter().filter(|matcher| {
                    match matcher.status{
                        Status::Failed => false,
                        Status::Running => true,
                        Status::Complete => true,
                    }
                }).collect();

                if ! all_matches.is_empty(){
                    memory.push(c);

                    println!("MEM PUSH: {}", c);
                }

                // For every matched item
                loop {
                    // Return the first item in the list if it is complete
                    let front_op = if ! all_matches.is_empty() {
                        let front: &Matcher = all_matches.front().unwrap();
                        match front.status {
                            Status::Complete => Some(all_matches.pop_front().unwrap()),
                            _ => None
                        }
                    }
                    else{
                        None
                    };
                    if let Some( front ) = front_op {
                        // Clear matchers that overlapped with the thang 
                        all_matches = all_matches.into_iter().filter(|matcher|{
                            if matcher.start_i >= front.get_clear_to_i() {
                                return true
                            }
                            false
                        }).collect();

                        if front.start_i > memory.get_index(){
                            let str_offset = front.start_i - memory.get_index() ;
                            let prev_str = memory.drain_chars(str_offset);
                            println!("MEM DRAIN PUSH: {}", prev_str);
                            to_str.push_str(&prev_str);
                        }

                        // Sync memory and update str with matcher replace
                        memory.set_index(front.end_i +1);  // ?? is on the last char, not after it
                        println!("MEM INDEX: {}", memory.start_i);
                        memory.drain_chars(front.from_seq_count);
                        to_str.push_str(front.to_seq);
                        println!("REP PUSH: {}", front.to_seq);

                    }
                    else{
                        break;
                    }
                }

                // If empty replace rest of the string
                if all_matches.is_empty(){
                    to_str.push_str(&memory.drain_all());
                    memory.set_index(c_index + 1);
                    println!("MEM INDEX: {}", memory.start_i);
                }
            }
            else{
                memory.set_index(c_index +1);
                println!("MEM INDEX: {}", memory.start_i);
                println!("MEM PUSH C: {}", c);
                to_str.push(c);
            }
            println!("CHAR: {}, LOOP COUNT {}", c, c_index);
        }

    to_str
    }
    use std::ops::Add;

}

struct StringCache{
    s: String,
    start_i: usize,
}
impl StringCache{

    pub fn new() -> Self{
        StringCache{
            s: "".to_string(),
            start_i: 0,
        }
    }

    pub fn drain_chars(&mut self, count: usize) -> String{
        println!("Current String: {}", self.s);
        println!("Drain_Chars (COUNT): {}", count);


        let mut ret_s: String = "".to_string();
        let mut chars_rem: Vec<char> = self.s.clone().chars().collect();
        let chars: Vec<char> = chars_rem.drain(..count).collect();

        self.s.clear();
        for c in chars_rem{
            self.s.push(c);
        }

        for c in chars{
            ret_s.push(c);
        }
        ret_s
    }

    pub fn drain_all(&mut self) -> String{
        let ret_s = self.s.clone();
        self.s.clear();
        return ret_s;
    }

    pub fn push(&mut self, c: char){
        self.s.push(c);
    }

    pub fn clear(&mut self){
        self.s.clear();
    }

    pub fn inc_index(&mut self){
        self.start_i += 1;
    }

    pub fn inc_index_by(&mut self, num: usize){
        self.start_i += num;
    }

    pub fn set_index(&mut self, index: usize){
        self.start_i = index;

    }

    pub fn get_index(&self) -> usize{
        return self.start_i;
    }


    pub fn count(&self) -> usize{
        self.s.chars().count()
    }
}

pub fn hash_to_vec_ordered_longest_shortest<'a>(seqs_hash: HashMap<&'a str, &'a str>) -> Vec<(&'a str, &'a str) >{
    let mut seqs_vec: Vec< (&str, &str) > = vec![];
    for key in seqs_hash.keys(){
        seqs_vec.push( (key, seqs_hash.get(key).unwrap()) );
    }
    seqs_vec.sort_by(|a, b| b.0.chars().count().cmp(&a.0.chars().count()));

    seqs_vec
}

pub enum Status{
    Complete,
    Running,
    Failed,
}
pub fn build_matches<'a>(start_index: usize, initial_char: char, seqs: &'a Vec<(&'a str, &'a str)> ) -> Option<Vec<Matcher>>{
    let mut matchers: Vec<Matcher> = vec![];
    for (from, to) in seqs{
        if let Some(c) = from.chars().next(){
            if c == initial_char{
                matchers.push(Matcher::new(from, to, start_index));
            }
        }
    }

    if matchers.is_empty(){
        return None;
    }
    return Some(matchers);
}


pub struct Matcher <'a> {
    status: Status,
    from_seq: Vec<char>,
    to_seq: &'a str,
    from_seq_count: usize,
    start_i: usize,
    end_i: usize,
    counter: usize,
}
impl<'a> Matcher <'a>{
    pub fn new(from_seq: &'a str, to_seq: &'a str, start_i: usize) -> Self{
        let from_seq_count = from_seq.chars().count();
        let from_seq: Vec<char> = from_seq.chars().collect();
        Self{
            status: Status::Running,
            from_seq: from_seq,
            to_seq: &to_seq,
            from_seq_count: from_seq_count,
            start_i: start_i,
            end_i: start_i + from_seq_count - 1,
            counter: 0,
        }
    }

    pub fn run(&mut self, c: char) -> Status{
        match self.status{
            Status::Running => self.next(c),
            Status::Complete => Status::Complete,
            Status::Failed => Status::Failed,   // Failed is never returned from match. Added for compiler reasons.
        }
    }

    // Matches the current char in the sequence with the char that is being read.
    // If the the matched char is the last in the sequence it returns Status::Complete
    // If the match fails it returns Status::Failed
    fn next(&mut self, c: char) -> Status{
        if self.from_seq[self.counter] == c{
            self.counter += 1;

            if self.counter == self.from_seq.len(){
                self.status = Status::Complete;
                return Status::Complete;
            }
            return Status::Running;
        }
        else{
            self.status = Status::Failed;
            return Status::Failed;
        }
    }

    pub fn get_clear_to_i(&self) -> usize{
        return self.start_i + self.from_seq_count;
    }
}





#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    
    use crate::*;
    #[test]
    pub fn drain_chars(){
        let mut sc = StringCache::new();
        sc.s = String::from("hippo");
        println!("Before drain: {}", sc.s);
        let ret_s = sc.drain_chars(5);
        println!("After drain: {}", sc.s);
        
        assert_eq!(ret_s, "hippo");
        //assert_eq!("ppo".to_string(), sc.s);
    }

    #[test]
    pub fn drain_all(){
        let mut sc = StringCache::new();
        sc.s = String::from("my word");
        let ret_s = sc.drain_all();

        assert_eq!(ret_s, "my word");
        assert_eq!("".to_string(), sc.s);
    }

    #[test]
    pub fn highest_complexity(){
        let mut replacement_seqs: HashMap<&str, &str> = HashMap::new();
        replacement_seqs.insert("a", "x");
        replacement_seqs.insert("bbbb c dddd eX", "!NO REPLACE");
        replacement_seqs.insert("c", "l");
        replacement_seqs.insert("c dddd", "y");
        replacement_seqs.insert("c dddd eeX", "!NO REPLACE");
        replacement_seqs.insert("gg ", "z ");
        replacement_seqs.insert("g", "r");
        replacement_seqs.insert("hh", "END");
        pr(&replacement_seqs);

        let from_s = "a bbbb cc dddd eee f gggggg hh";
        let expect_s = "x bbbb ly eee f rrrrz END";
        let new_s = Replacer::run(from_s, replacement_seqs);
        println!("Orig: {}", from_s);
        println!("New String: {:?}", new_s);

        assert_eq!(expect_s, new_s);
    }

    #[test]
    pub fn longest_collapse_three_shorters(){
        let mut replacement_seqs: HashMap<&str, &str> = HashMap::new();
        replacement_seqs.insert("wow is my fox", "mmmmmmmmm");
        replacement_seqs.insert("wow", "aaaa");
        replacement_seqs.insert("is", "bb");
        replacement_seqs.insert("my", "cc");
        pr(&replacement_seqs);

        let from_s = "wow is my foo";
        let expect_s = "aaaa bb cc foo";
        let new_s = Replacer::run(from_s, replacement_seqs);
        println!("Orig: {}", from_s);
        println!("New String: {:?}", new_s);

        assert_eq!(expect_s, new_s);
    }

    #[test]
    pub fn longest_collapse_two_shorters(){
        let mut replacement_seqs: HashMap<&str, &str> = HashMap::new();
        replacement_seqs.insert("wow is my fox", "mmmmmmmmm");
        replacement_seqs.insert("is", "bb");
        replacement_seqs.insert("my", "cc");
        pr(&replacement_seqs);

        let from_s = "wow is my foo";
        let expect_s = "wow bb cc foo";
        let new_s = Replacer::run(from_s, replacement_seqs);
        println!("Orig: {}", from_s);
        println!("New String: {:?}", new_s);

        assert_eq!(expect_s, new_s);
    }

    #[test]
    pub fn longest_collapse_replace_shorter(){
        let mut replacement_seqs: HashMap<&str, &str> = HashMap::new();
        replacement_seqs.insert("present", "future");
        replacement_seqs.insert("present txxxxx", "past");
        pr(&replacement_seqs);

        let from_s = "a present tense foo";
        let expect_s = "a future tense foo";
        let new_s = Replacer::run(from_s, replacement_seqs);
        println!("Orig: {}", from_s);
        println!("New String: {:?}", new_s);

        assert_eq!(expect_s, new_s);
    }

    #[test]
    pub fn longest_replace(){
        let mut replacement_seqs: HashMap<&str, &str> = HashMap::new();
        replacement_seqs.insert("present", "future");
        replacement_seqs.insert("present tense", "past");
        pr(&replacement_seqs);

        let from_s = "a present tense foo";
        let expect_s = "a past foo";
        let new_s = Replacer::run(from_s, replacement_seqs);
        println!("Orig: {}", from_s);
        println!("New String: {:?}", new_s);

        assert_eq!(expect_s, new_s);
    }

    #[test]
    pub fn multiple_seperate_replace(){
        let mut replacement_seqs: HashMap<&str, &str> = HashMap::new();
        replacement_seqs.insert("a", "it was");
        replacement_seqs.insert("present", "past");
        pr(&replacement_seqs);

        let from_s = "a present tense foo";
        let expect_s = "it was past tense foo";
        let new_s = Replacer::run(from_s, replacement_seqs);
        println!("Orig: {}", from_s);
        println!("New String: {:?}", new_s);

        assert_eq!(expect_s, new_s);
    }

    #[test]
    pub fn single_replace(){
        let mut replacement_seqs: HashMap<&str, &str> = HashMap::new();
        replacement_seqs.insert("a", "the");
        pr(&replacement_seqs);

        let from_s = "a present tense foo";
        let expect_s = "the present tense foo";
        let new_s = Replacer::run(from_s, replacement_seqs);
        println!("Orig: {}", from_s);
        println!("New String: {:?}", new_s);

        assert_eq!(expect_s, new_s);
    }

    #[test]
    pub fn hashmap_stays_in_order() {
        let mut map: HashMap<&str, &str> = HashMap::new();
        map.insert("ham", "cheese");
        map.insert("foo", "boo");
        assert!(map.get("ham").unwrap() == &"cheese");
        assert!(map.keys().len() == 2);
    }
    
    pub fn pr(map: &HashMap<&str, &str>){
        println!("Replacements: {:?}", map);
    }

    // While interating through a string the match will build a string of all characters that have been matched.
    #[test]
    pub fn matcher_works(){
        let from_match = "foo";
        let to_match = "bar";
        let mut sentance = "this is foo bar".chars();
        let mut matched_str = String::from("");
        let mut matcher: Option<Matcher> = Option::None;

        let mut count = 0;
        while let Some(c) = sentance.next() {
            if c == 'f'{
                matcher = Option::Some(Matcher::new(from_match, to_match, count));
            }

            let status = match matcher.as_mut(){
                Some(m) => m.run(c),
                None => Status::Failed,
            };


            match status {
                Status::Running => matched_str = format!("{}{}", matched_str, c),
                Status::Complete => {
                    matched_str = format!("{}{}", matched_str, c);
                    matcher = Option::None;
                },
                Status::Failed => matcher = Option::None,
            }
            println!("to match: {}", from_match);
            println!("matched: {}", matched_str);

            count += 1;
        }

            assert!(matched_str == from_match);
    }

    #[test]
    pub fn hashmap_to_vector_sort_longest(){
        let mut seqs_hash: HashMap<&str, &str> = HashMap::new();
        seqs_hash.insert("a super duper long foo", "a super duper long bar");
        seqs_hash.insert("fooest", "barest");
        seqs_hash.insert("foo", "bar");
        seqs_hash.insert("fooer", "barer");
        seqs_hash.insert("a very long foo", "a very long foo");

        let mut seqs_vec: Vec< (&str, &str) > = vec![];
        for key in seqs_hash.keys(){
            seqs_vec.push( (key, seqs_hash.get(key).unwrap()) );
        }
        seqs_vec.sort_by(|a, b| b.0.len().cmp(&a.0.len()));
        println!{"{:?}", seqs_vec};

        assert!(seqs_vec[0].0 == "a super duper long foo");
        assert!(seqs_vec[1].0 == "a very long foo");
        assert!(seqs_vec[2].0 == "fooest");
        assert!(seqs_vec[3].0 == "fooer");
        assert!(seqs_vec[4].0 == "foo");

    }
}