pub struct Action {
    pub index: u8,
    pub message: String,
}

impl Action {
    fn print(&self) {
        println!("{}. {}", self.index, self.message);
    }
}

pub struct ActionList {
    pub list: Action,
    pub add: Action,
    pub remove: Action,
    pub exit: Action,
    pub help: Action
}

pub struct Actions {
    pub actions_list: ActionList,
}

impl Actions {
    pub fn init() -> Self {
        Actions {
            actions_list: ActionList {
                list: Action {
                    index: 1,
                    message: String::from("list all todos"),
                },
                add: Action {
                    index: 2,
                    message: String::from("add new todo"),
                },
                remove: Action {
                    index: 3,
                    message: String::from("remove todo"),
                },
                help: Action {
                    index: 4,
                    message: String::from("help"),
                },
                exit: Action {
                    index: 5,
                    message: String::from("exit"),
                },
            },
        }
    }

    pub fn print_actions(&self) {
        self.actions_list.list.print();
        self.actions_list.add.print();
        self.actions_list.remove.print();
        self.actions_list.help.print();
        self.actions_list.exit.print();
    }
}
