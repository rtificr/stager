use fancy_regex::Regex;
use indexmap::IndexMap;
use crate::element::Element;
use crate::element_holder::ElementHolder;
use crate::error::err_code;

pub struct Act {
    pub title: String,
    pub author: String,
    pub desc: String,
    pub elements: IndexMap<String, Element>,
}

pub enum Desc {
    Title = 0,
    Author = 1,
    Desc = 2,
}

impl Act {
    pub fn new(mut act_str: String) -> Result<Act, String> {
        let mut act = Act {
            //desc_count: 3,
            title: String::new(),
            author: String::new(),
            desc: String::new(),
            elements: IndexMap::new(),
        };

        let descs = get_descriptors(&mut act_str)?;

        act.title = descs[Desc::Title as usize].clone();
        act.author = descs[Desc::Author as usize].clone();
        act.desc = descs[Desc::Desc as usize].clone();

        match process_act(act_str){
            Ok(content) => act.elements = content,
            Err(e) => return Err(format!("Unable to make act:\n{e}"))
        }

        Ok(act)
    }
}

fn get_descriptors(act_str: &mut String) -> Result<[String; 3], String> {
    let patterns = [
        Regex::new(r"(?i)((?:(^|\\))(?:T\s*:\s*)(.*?)(\\))").map_err(|e| err_code("100", e))?,
        Regex::new(r"(?i)((?:(^|\\))(?:A\s*:\s*)(.*?)(\\))").map_err(|e| err_code("101", e))?,
        Regex::new(r"(?i)((?:(^|\\))(?:D\s*:\s*)(.*?)(\\))").map_err(|e| err_code("102", e))?
    ];

    let filter = Regex::new(r"(?:(\\|^)\w*\s*:\s*)(.*)(?=\\)").map_err(|e| err_code("103", e))?;

    let mut results: [String; 3] = Default::default();
    let descriptors = ["title", "author", "description"];

    //a lot of this error handling shouldn't ever be needed
    //I am not throwing errors because they are harmless; I'm only making them known
    for i in 0..patterns.len() {
        //check if descriptor exists; if so...
        match patterns[i].find(&act_str.clone()) {
            Ok(Some(m1)) => {
                //get content of descriptor
                match filter.captures(&m1.as_str()) {
                    Ok(Some(m2)) => {
                        if let Some(match_group) = m2.get(2) {
                            //a lot of conversion going on here :|
                            results[i] = match_group.as_str().to_string();
                            //remove descriptors from act
                            *act_str = str::replace(act_str.as_str(), m1.as_str(), "");
                        } else {
                            eprintln!("ERROR: {} formatting incorrect; unable to get descriptor: {}", descriptors[i], m1.as_str());
                        }
                    }
                    Ok(None) => eprintln!("ERROR: {} formatting incorrect; unable to filter out descriptor", descriptors[i]),
                    Err(e) => {
                        return Err(err_code(&format!("11{}a", i), e))
                    }
                }
            }
            Ok(None) => eprintln!("No {} found (non-critical)", descriptors[i]),
            Err(e) => {
                return Err(err_code(&format!("11{}b", i), e))
            }
        }

    }

    Ok(results)
}

fn process_act(act: String) -> Result<IndexMap<String, Element>, String> {
    let element_str_vec = act.split("}").collect::<Vec<&str>>();

    let mut elements: IndexMap<String, Element> = IndexMap::new();

    let mut index: usize = 0;

    for element_str in element_str_vec {
        if element_str == "" {continue};
        let element_holder;
        match ElementHolder::new(element_str.to_string(), index){
            Ok(elh) => element_holder = elh,
            Err(e) => {
                return Err(e);
            }
        }
        index += 1;

        elements.insert(element_holder.name, element_holder.element);
    }

    Ok(elements)
}