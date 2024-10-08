use std::io;
use termion::input::TermRead;

pub struct Launcher
{
    nodes : Vec<LaunchNode>,
    process : Vec<std::process::Child>
}
impl Launcher {
    pub fn new()->Self
    {
        Launcher { nodes: Vec::new() , process : Vec::new()}
    }

    pub fn add(&mut self, node : LaunchNode)
    {
        self.nodes.push(node);
    }

    pub fn launch(&mut self)
    {
        for node in self.nodes.clone()
        {
            if node.node_name.clone() != None && node.config_file_path.clone() != None
            {
                let config_file_path_ = format!("{}/{}", node.pkg_name, node.config_file_path.unwrap());

                let _process = std::process::Command::new("cargo")
                    .arg("run")
                    .arg("-p")
                    .arg(node.pkg_name)
                    .arg("--bin")
                    .arg(node.node_name.unwrap())
                    .arg(config_file_path_).spawn().unwrap();

                self.process.push(_process);
            }
            else if node.node_name.clone() != None && node.config_file_path.clone() == None
            {
                let _process = std::process::Command::new("cargo")
                    .arg("run")
                    .arg("-p")
                    .arg(node.pkg_name)
                    .arg("--bin")
                    .arg(node.node_name.unwrap()).spawn().unwrap();

                    self.process.push(_process);
            }
            else if node.node_name.clone() == None && node.config_file_path.clone() != None
            {
                let config_file_path_ = format!("{}/{}", node.pkg_name, node.config_file_path.unwrap());

                let _process = std::process::Command::new("cargo")
                    .arg("run")
                    .arg("-p")
                    .arg(node.pkg_name)
                    .arg(config_file_path_).spawn().unwrap();

                    self.process.push(_process);
            }
            else {
                let _process = std::process::Command::new("cargo")
                    .arg("run")
                    .arg("-p")
                    .arg(node.pkg_name).spawn().unwrap();

                    self.process.push(_process);
            }
        }

        let stdin = io::stdin();

        for c in stdin.keys()
        {
            match c {
                Ok(termion::event::Key::Ctrl('c'))=>{
                    let pro_nm = self.process.len();

                    for i in 0..pro_nm
                    {
                        let _ = self.process[i].kill();
                    }
                }
                Err(_)=>{

                }
                _=>{}
            }
        }
    }
}


#[derive(Clone)]
pub struct LaunchNode
{
    pub pkg_name : String,
    pub node_name : Option<String>,
    pub config_file_path : Option<String>
}

impl LaunchNode {
    pub fn new(pkg_name : &str)->Self
    {
        LaunchNode { pkg_name: pkg_name.to_string(), node_name: None, config_file_path: None }
    }
    pub fn set_node(&mut self, node_name : &str)
    {
        self.node_name = Some(node_name.to_string())
    }
    pub fn set_config_file(&mut self, file_path : &str)
    {
        self.config_file_path = Some(file_path.to_string())
    }
}