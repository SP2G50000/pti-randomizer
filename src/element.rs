#[derive(Clone)]
pub enum ElementType {
    Normal,
    AutoPortal,
    ConversionGel,
    Laser,
    Pellet,
}

#[derive(Clone)]
pub enum AllowValue {
    Everywhere,
    Rexaura,
    Portal2,
}

//A test element.
#[derive(Clone)]
pub struct TestElement {
    pub name: String,
    pub allowed: AllowValue,
    pub element_type: ElementType,
    pub is_major: bool,
    pub is_input: bool,
}

impl TestElement {
    pub fn minor(string: &str, input: bool) -> TestElement {
        TestElement { name: String::from(string), allowed: AllowValue::Everywhere, element_type: ElementType::Normal, is_major: false, is_input: input }
    }

    // pub fn major(string: &str) -> TestElement {
    //     TestElement { name: String::from(string), allowed: AllowValue::Everywhere, element_type: ElementType::Normal, is_major: true, is_input: false }
    // }

    pub fn rex_minor(string: &str, input: bool) -> TestElement {
        TestElement { name: String::from(string), allowed: AllowValue::Rexaura, element_type: ElementType::Normal, is_major: false, is_input: input }
    }

    // pub fn rex_major(string: &str) -> TestElement {
    //     TestElement { name: String::from(string), allowed: AllowValue::Rexaura, element_type: ElementType::Normal, is_major: true, is_input: false }
    // }
    
    pub fn p2_minor(string: &str, input: bool) -> TestElement {
        TestElement { name: String::from(string), allowed: AllowValue::Portal2, element_type: ElementType::Normal, is_major: false, is_input: input }
    }

    pub fn p2_major(string: &str) -> TestElement {
        TestElement { name: String::from(string), allowed: AllowValue::Portal2, element_type: ElementType::Normal, is_major: true, is_input: false }
    }
    
    pub fn laser(string: &str, input: bool) -> TestElement {
        TestElement { name: String::from(string), allowed: AllowValue::Portal2, element_type: ElementType::Laser, is_major: false, is_input: input }
    }

    pub fn laser_emitter() -> TestElement {
        TestElement { name: String::from("Laser Emitter"), allowed: AllowValue::Portal2, element_type: ElementType::Laser, is_major: true, is_input: false }
    }

    pub fn pellet(string: &str, input: bool) -> TestElement {
        TestElement { name: String::from(string), allowed: AllowValue::Rexaura, element_type: ElementType::Pellet, is_major: false, is_input: input }
    }

    pub fn pellet_emitter() -> TestElement {
        TestElement { name: String::from("Laser Emitter"), allowed: AllowValue::Rexaura, element_type: ElementType::Pellet, is_major: true, is_input: false }
    }

    pub fn auto_portal() -> TestElement {
        TestElement { name: String::from("Auto Portal"), allowed: AllowValue::Everywhere, element_type: ElementType::AutoPortal, is_major: false, is_input: false }
    }

    pub fn conversion_gel() -> TestElement {
        TestElement { name: String::from("Conversion Gel"), allowed: AllowValue::Portal2, element_type: ElementType::ConversionGel, is_major: true, is_input: false }
    }
}