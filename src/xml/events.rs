use common;
use common::{Name, Attribute, XmlVersion};

#[deriving(Eq, Clone)]
pub enum XmlEvent {
    StartDocument {
        version: XmlVersion,
        encoding: ~str,
        standalone: Option<bool>
    },
    EndDocument,
    ProcessingInstruction { 
        name: ~str, 
        data: Option<~str> 
    },
    StartElement { 
        name: Name,
        attributes: ~[Attribute]
    },
    EndElement {
        name: Name
    },
    CData(~str),
    Comment(~str),
    Characters(~str),
    Whitespace(~str),
    Error(common::Error)
}


pub enum Standalone { SDYes, SDNo }
