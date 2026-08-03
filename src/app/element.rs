use crate::app::function::Run;

pub enum Diagram<R: Run, AE, AL> {
    List(List<AL, Self>),
    Element(Element<R, AE>),
}

pub struct List<AL, T> {
    pub id: usize,
    pub additional_data: AL,
    pub data: Vec<T>,
}

pub struct Element<R: Run, AE> {
    pub additional_data: AE,
    pub run: R,
}

unsafe impl<R: Run, AE, AL> Send for Diagram<R, AE, AL> {}
unsafe impl<AL, T> Send for List<AL, T> {}

impl<R: Run, AE, AL> Diagram<R, AE, AL> {
    pub fn find_list_mut(&mut self, find_id: usize) -> Option<&mut List<AL, Diagram<R, AE, AL>>> {
        match self {
            Diagram::List(list) => {
                if list.id.clone() == find_id {
                    Some(list)
                } else {
                    list.data.iter_mut().find_map(|diag| diag.find_list_mut(find_id))
                }
            }
            Self::Element { .. } => None
        }
    }    
    
    pub fn find_list(&self, find_id: usize) -> Option<&List<AL, Diagram<R, AE, AL>>> {
        match self {
            Diagram::List(list) => {
                if list.id.clone() == find_id {
                    Some(list)
                } else {
                    list.data.iter().find_map(|diag| diag.find_list(find_id))
                }
            }
            Self::Element { .. } => None
        }
    }
    
    pub fn get_len(&self, find_id: usize) -> Option<usize> {
        self.find_list(find_id).map(|diag| diag.data.len())
    }
}