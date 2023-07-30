#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod task_list {

    #[derive(Debug, Clone, scale::Encode, scale::Decode, PartialEq)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct TaskList {
        pub title: String,
        pub description: String,
    }

    #[ink(storage)]
    pub struct Task {
        pub list: Vec<TaskList>,
    }

    impl Task {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { list: Vec::new() }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self {
                list: Default::default(),
            }
        }

        #[ink(message)]
        pub fn add(&mut self, title: String, description: String) {
            self.list.push(TaskList { title, description })
        }

        #[ink(message)]
        pub fn update_task(&mut self, index: u32, title: String, description: String) {
            let index_usize: usize = index as usize;
            self.list[index_usize].title = title;
            self.list[index_usize].description = description;
        }

        #[ink(message)]
        pub fn delete_task(&mut self, index: u32) {
            let index_usize: usize = index as usize;
            self.list.swap_remove(index_usize);
        }

        #[ink(message)]
        pub fn get_list(&self) -> Vec<TaskList> {
            self.list.clone()
        }

        #[ink(message)]
        pub fn get_task(&self, index: u32) -> Option<TaskList> {
            let index_usize: usize = index as usize;
            self.list.get(index_usize).cloned()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::task_list::Task;

    #[test]
    fn test_add() {
        // Prueba con título y descripción cortos
        {
            println!("Ejecutando prueba con título y descripción cortos");
            let mut task = Task::default();

            task.add("Test".to_owned(), "Short description".to_owned());

            let get_tasks = task.get_task(0);

            assert_eq!(get_tasks.as_ref().unwrap().title, "Test");
            assert_eq!(get_tasks.as_ref().unwrap().description, "Short description");
        }

        // Prueba con título y descripción largos
        {
            println!("Prueba con título y descripción largos");
            let mut task = Task::default();

            task.add(
                "Blockchain".to_owned(),
                "A blockchain is a distributed ledger with growing lists of records (blocks) that are securely linked together via cryptographic hashes."
                    .to_owned(),
            );

            let get_tasks = task.get_task(0);

            assert_eq!(get_tasks.as_ref().unwrap().title, "Blockchain");
            assert_eq!(
                get_tasks.as_ref().unwrap().description,
                "A blockchain is a distributed ledger with growing lists of records (blocks) that are securely linked together via cryptographic hashes."
            );
        }

        // Prueba de tarea duplicada
        {
            println!("Prueba con título y descripción largos duplicados");
            let mut task = Task::default();

            task.add(
                "Blockchain".to_owned(),
                "A blockchain is a distributed ledger with growing lists of records (blocks) that are securely linked together via cryptographic hashes."
                    .to_owned(),
            );

            let get_tasks = task.get_task(0);

            assert_eq!(get_tasks.as_ref().unwrap().title, "Blockchain");
            assert_eq!(
                get_tasks.as_ref().unwrap().description,
                "A blockchain is a distributed ledger with growing lists of records (blocks) that are securely linked together via cryptographic hashes."
            );
        }
    }


    #[test]
    fn test_get_task() {
        // Prueba con una tarea agregada previamente
        {
            println!("Ejecutando prueba con la tarea agregada");
            let mut task = Task::default();

            task.add("Test".to_owned(), "Short description".to_owned());

            let get_tasks = task.get_task(0);

            assert_eq!(get_tasks.as_ref().unwrap().title, "Test");
            assert_eq!(get_tasks.as_ref().unwrap().description, "Short description");

            println!("Task: {:?}", get_tasks);
        }

        // Prueba con una lista vacia
        {
            println!("Ejecutando prueba con lista vacia");
            let task = Task::default();

            let get_tasks = task.get_task(0);

            assert_eq!(get_tasks, None);

            println!("Task: {:?}", get_tasks);
        }

        // Prueba con un indice fuera del rango del vector
        {
            println!("Ejecutando prueba con un indice fuera del rango del vector");
            let mut task = Task::default();

            task.add("Test".to_owned(), "test".to_owned());

            let get_tasks = task.get_task(1);

            assert_eq!(get_tasks, None);

            println!("Task: {:?}", get_tasks);
        }
    }



    #[test]
    fn test_get_list() {
        // Prueba de la lista de tarea
        println!("Ejecutando prueba para mostrar la lista de tarea");
        let mut task = Task::default();

        println!();

        task.add("Test".to_owned(), "test1".to_owned());

        task.add("Test".to_owned(), "test2".to_owned());

        task.add("Test".to_owned(), "test3".to_owned());

        task.add("Test".to_owned(), "test4".to_owned());

        let get_task_lists = task.get_list();

        assert_eq!(get_task_lists[0].title, "Test");
        assert_eq!(get_task_lists[0].description, "test1");
    
        assert_eq!(get_task_lists[1].title, "Test");
        assert_eq!(get_task_lists[1].description, "test2");
    
        assert_eq!(get_task_lists[2].title, "Test");
        assert_eq!(get_task_lists[2].description, "test3");

        println!("Task list: {:?}", get_task_lists);
    }


    #[test]
    fn test_update_task() {

        println!("Ejecutando prueba de actualizacion de tarea");
        let mut task = Task::default();

        task.add("Test".to_owned(), "test".to_owned());
        
    }

}
