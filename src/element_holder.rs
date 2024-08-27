use crate::choice::Choice;
use crate::element::Element;
use crate::error::err_code;
use fancy_regex::Regex;

pub struct ElementHolder {
    pub name: String,
    pub element: Element,
}

impl ElementHolder {
    pub fn default() -> ElementHolder{
        ElementHolder {
            name: String::new(),
            element: Element::default(),
        }
    }
    pub fn new(input: String, element_index: usize) -> Result<ElementHolder, String> {
        let pattern_name = Regex::new(r"(?:^)(.*?)(?=:)").map_err(|e| err_code("100", e))?;
        let pattern_content = Regex::new(r"(?<=:)(.*?)(?=\{)").map_err(|e| err_code("101", e))?;
        let pattern_body = Regex::new(r"(?<=\{).*").map_err(|e| err_code("102", e))?;

        let name = pattern_name.find(&input)
            .map_err(|e| err_code("110", e))?
            .map(|m| m.as_str().to_string())
            .ok_or_else(|| format!("Element #{element_index}: name not found"))?
            .trim().to_string();

        let content = pattern_content.find(&input)
            .map_err(|e| err_code("111", e))?
            .map(|m| m.as_str().to_string())
            .ok_or_else(|| format!("Element #{element_index}: content not found"))?
            .trim().to_string();

        let mut choices: Vec<Choice> = Vec::new();

        let body_match = pattern_body.find(&input)
            .map_err(|e| err_code("112", e))?;

        if let Some(m) = body_match
        {
            let mut choices_str: Vec<String> = m.as_str()
                .split('\\')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();

            if choices_str.len() % 2 != 0 {
                return Err(format!("Incompatible amount of choices and destinations in element '{name}'"));
            }

            for i in 0..(choices_str.len()/2) {
                let text = choices_str[i * 2].clone();
                let dest = choices_str[(i * 2) + 1].clone();

                let choice = Choice::new(text, dest);
                choices.push(choice);
            }
        } else {
            return Err(format!("Element '{name}': content not found"))
        }

        Ok(ElementHolder {
            name,
            element: Element::new(content, choices),
        })
    }
}