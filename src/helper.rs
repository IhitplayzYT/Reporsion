pub mod Helper{
    use std::{collections::HashSet, error::Error, fs, ops::DerefMut, path::{Path, PathBuf}};

    pub fn spread(path: impl AsRef<Path>,sandbox: impl AsRef<Path>) -> Result<(),Box<dyn Error>>{
        let mut no_children = true;

        for dir in fs::read_dir(path)?{
            let dir = dir.unwrap();
            let dpath = dir.path().canonicalize().unwrap();
            if dir.file_type().unwrap().is_dir(){
                no_children = false;
                spread(&dpath,sandbox.as_ref()).unwrap();                
            }
            
            if no_children{
                let mut fset: HashSet<PathBuf> = HashSet::new();
                
                fset.insert(PathBuf::from(".git/"));
                fset.insert(PathBuf::from(".gitignore"));
                build_gi(&dpath,&fset,sandbox.as_ref());  
            }

        }
        Ok(())
    }

    pub fn build_gi(cwd: &PathBuf,st: &HashSet<PathBuf>,sandbox: impl AsRef<Path>){ 
        //  a/b/c
        let ed_cwd = PathBuf::from(cwd.iter().last().unwrap());
        let (g_pth,gi_pth) =  (cwd.join(".git/"),cwd.join(".gitignore"));
        if g_pth.exists(){
            if let Some(old) = fs::read_to_string(&gi_pth).ok(){
                let mut buff = old.clone(); 
                let already_in = old.split("\n").filter_map(|x| if !x.trim().is_empty() {Some(x)}else{None}).collect::<HashSet<&str>>();
                for ele in st.iter() {
                    let ele_str  = ele.to_str().unwrap();
                    if already_in.contains(ele_str){
                        continue;
                    }
                    buff += ele_str;
                    buff += "\n";
                }
                fs::write(gi_pth, &buff).unwrap();

                let mut n_set: HashSet<PathBuf> = st.iter().map(|x| ed_cwd.join(x)).collect();
                n_set.insert(PathBuf::from(".git/"));
                n_set.insert(PathBuf::from(".gitignore"));

                if cwd.strip_prefix(sandbox.as_ref()).unwrap().as_os_str().is_empty(){return;}
                if let Some(par) = cwd.parent(){
                    build_gi(&par.to_owned(), &n_set,sandbox);
                }
            }else{
                let mut buff = String::new();
                for ele in st.iter() {
                    let ele_str  = ele.to_str().unwrap();
                    buff += ele_str;
                    buff += "\n";
                }
                fs::write(gi_pth, &buff).unwrap();

                let mut n_set: HashSet<PathBuf> = st.iter().map(|x| ed_cwd.join(x)).collect();
                n_set.insert(PathBuf::from(".git/"));
                n_set.insert(PathBuf::from(".gitignore"));
                if cwd.strip_prefix(sandbox.as_ref()).unwrap().as_os_str().is_empty(){return;}
                if let Some(par) = cwd.parent(){
                    build_gi(&par.to_owned(), &n_set,sandbox);
                }
            }
        }else{
            // Cur dir isnt git dir
            let n_set: HashSet<PathBuf> = st.iter().map(|x| ed_cwd.join(x)).collect();
            if let Some(par) = cwd.parent(){
                build_gi(&par.to_owned(), &n_set,sandbox);
            }
        }

    }

}