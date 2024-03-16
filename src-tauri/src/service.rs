use tauri::Manager;

struct Todo {
    tasks: Vec<String>,
}

impl Todo {
    fn new() -> Todo {
        Todo { tasks: Vec::new() }
    }

    fn add_task(&mut self, task: String) {
        self.tasks.push(task);
    }

    fn remove_task(&mut self, index: usize) {
        if index < self.tasks.len() {
            self.tasks.remove(index);
        }
    }

    fn update_task(&mut self, index: usize, new_task: String) {
        if index < self.tasks.len() {
            self.tasks[index] = new_task;
        }
    }

    fn get_tasks(&self) -> Vec<String> {
        self.tasks.clone()
    }
}

#[tauri::command]
fn add_task(todo: tauri::State<Todo>, task: String) {
    todo.add_task(task);
}

#[tauri::command]
fn remove_task(todo: tauri::State<Todo>, index: usize) {
    todo.remove_task(index);
}

#[tauri::command]
fn update_task(todo: tauri::State<Todo>, index: usize, new_task: String) {
    todo.update_task(index, new_task);
}

#[tauri::command]
fn get_tasks(todo: tauri::State<Todo>) -> Vec<String> {
    todo.get_tasks()
}

fn main() {
    let todo = Todo::new();

    tauri::Builder::default()
        .manage(todo)
        .invoke_handler(tauri::generate_handler![
            add_task,
            remove_task,
            update_task,
            get_tasks
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
