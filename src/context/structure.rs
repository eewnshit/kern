use std::{any::Any, sync::Mutex};

#[derive(Debug)]
pub struct KData<'a> {
    pub key: &'a str,
    pub value: &'a dyn Any,
}


impl<'a> Clone for KData<'a> {
    fn clone(&self) -> Self {
        KData {
            key: self.key,
            value: self.value,  // Clona o valor (assumindo que `value` é clonável)
        }
    }
}

#[derive(Debug)]
pub struct KTable<'a> {
    pub size: usize,
    pub cycles: Vec<Option<KData<'a>>>,  // Mudança: Option<KData> permite slots vazios (None)
}

impl<'a> KData<'a> {
    pub fn new(key: &'a str, value: &'a dyn Any) -> Self {
        KData { key, value }
    }
}

impl<'a> KTable<'a> {
    pub fn new(size: usize) -> Self {
        KTable {
            size,
            cycles: vec![None; size],
        }
    }

    pub fn insert(&mut self, key: &'a str, value: &'a dyn Any) {
        let index: usize = self.hash(key, self.size as i32) as usize;
        let mut x: i32 = 0;
    
        while let Some(slot) = self.cycles.get(self.probing(index, x, self.size) as usize) {
            if slot.is_none() {
                break;
            }
            x += 1;
        }
    
        let final_index: usize = self.probing(index, x, self.size) as usize;
        self.cycles[final_index] = Some(KData::new(key, value));
    }

    pub fn get<T>(&self, key: &'a str) -> Option<&T> 
    where T: 'a + 'static, 
    {
        let index: usize = self.hash(key, self.size as i32) as usize;
        let mut x: i32 = 0;

        while let Some(slot) = self.cycles.get(self.probing(index, x, self.size) as usize) {
            if let Some(cycle) = slot {
                if cycle.key == key {
                    if let Some(value) = cycle.value.downcast_ref::<T>() {
                        return Some(value);
                    }
                }
            }
            x += 1;
        }

        None
    }

    pub fn remove(&mut self, key: &'a str) {
        let index: usize = self.hash(key, self.size as i32) as usize;
        let mut x: i32 = 0;
        let probing_index: usize = self.probing(index as usize, x, self.size) as usize;

        // Realiza o probing para encontrar o item a ser removido
        while let Some(_slot) = self.cycles.get_mut(probing_index) {
            let current_index: usize = self.probing(index, x, self.size) as usize;
            if let Some(cycle) = &mut self.cycles[current_index] {
                if cycle.key == key {
                    self.cycles[current_index] = None;  // Marca o slot como vazio
                    return;
                }
            }
            x += 1;
        }
    }

    fn hash(&self, key: &'a str, size: i32) -> i32 {
        let mut hash_value: i32 = 0;
        for i in 0..key.len() {
            let ch = key.chars().nth(i).unwrap();
            hash_value = (hash_value * 31 + ch as i32) % size;
        }
        hash_value
    }

    fn probing(&self, index: usize, x: i32, size: usize) -> i32 {
        // Linear probing: (index + x) % size
        ((index + x as usize) % size) as i32
    }
}