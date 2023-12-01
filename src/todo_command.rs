pub enum TodoCommand {
    New {
        list: Vec<String>,
        file: Option<String>,
    },
    Append {
        list: Vec<String>,
        file: Option<String>,
    },
    Delete {
        list: Vec<usize>,
        file: Option<String>,
    },
    Done {
        list: Vec<usize>,
        file: Option<String>,
    },
    Edit {
        index: usize,
        string: String,
        file: Option<String>,
    },
    Clear {
        file: Option<String>,
    },
    Read {
        file: Option<String>,
        include_done: bool,
    },
}

impl TodoCommand {
    pub fn new(args: &Vec<String>) -> Result<TodoCommand, String> {
        if args.len() < 2 {
            return Err("Not enough arguments!".to_string());
        }

        match args[1].as_str() {
            "new" | "append" => {
                // Find the -o tag in the front of command
                let o_tag_pos = args.iter().position(|x| x.as_str().eq("-o"));
                let output_path_str = o_tag_pos.and_then(|i| args.get(i + 1));

                if let None = o_tag_pos {
                    return Ok(TodoCommand::New {
                        list: args
                            .iter()
                            .skip(2)
                            .map(|s| s.clone())
                            .collect::<Vec<String>>(),
                        file: None,
                    });
                }

                if let None = o_tag_pos.and(output_path_str) {
                    Err("Found -o tag but no file was specified".to_owned())
                } else {
                    // There are three conditions:
                    // 1. <command> -o <arg> ... "in the front"
                    // 2. <command> ... -o <arg> ... "in the middle"
                    // 3. <command> ... -o <arg> "in the back"

                    let (list, file) = match o_tag_pos.unwrap() {
                        // 1
                        2 => (
                            args.iter()
                                .skip(4)
                                .map(|s| s.clone())
                                .collect::<Vec<String>>(),
                            Some(output_path_str.unwrap().clone()),
                        ),
                        // 3
                        pos if pos == args.len() - 2 => (
                            args.split_at(pos)
                                .0
                                .iter()
                                .skip(2)
                                .map(|s| s.to_owned())
                                .collect::<Vec<String>>(),
                            Some(output_path_str.unwrap().to_owned()),
                        ),
                        // 2
                        o_tag_pos => {
                            // Split vecs at "-o" tag
                            let (before_o, _) = args.split_at(o_tag_pos);

                            // Split vecs one after "-o" tag
                            let (_, one_after_o) = args.split_at(o_tag_pos + 2);

                            println!("after_o: {:?}", one_after_o);

                            // Join those arrays
                            let joined = before_o
                                .iter()
                                .chain(one_after_o.iter())
                                .map(|s| s.clone())
                                .skip(2)
                                .collect::<Vec<String>>();

                            println!("joined: {:?}", joined);

                            (joined, Some(output_path_str.unwrap().clone()))
                        }
                    };
                    if args[1].as_str().eq("new") {
                        Ok(TodoCommand::New { list, file })
                    } else {
                        Ok(TodoCommand::Append { list, file })
                    }
                }
            }
            "delete" | "done" => {
                // Find the -o tag in the front of command
                let o_tag_pos = args.iter().position(|x| x.as_str().eq("-o"));
                let output_path_str = o_tag_pos.and_then(|i| args.get(i + 1));

                if let None = o_tag_pos {
                    return Ok(TodoCommand::New {
                        list: args
                            .iter()
                            .skip(2)
                            .map(|s| s.clone())
                            .collect::<Vec<String>>(),
                        file: None,
                    });
                }

                if let None = o_tag_pos.and(output_path_str) {
                    Err("Found -o tag but no file was specified".to_owned())
                } else {
                    // There are three conditions:
                    // 1. <command> -o <arg> ... "in the front"
                    // 2. <command> ... -o <arg> ... "in the middle"
                    // 3. <command> ... -o <arg> "in the back"

                    let (list, file) = match o_tag_pos.unwrap() {
                        // 1
                        2 => (
                            args.iter()
                                .skip(4)
                                .map(|s| s.parse::<usize>())
                                .map(|r| r.expect("Failed to parse integer"))
                                .collect::<Vec<usize>>(),
                            Some(output_path_str.unwrap().clone()),
                        ),
                        // 3
                        pos if pos == args.len() - 2 => (
                            args.split_at(pos)
                                .0
                                .iter()
                                .skip(2)
                                .map(|s| s.parse::<usize>())
                                .map(|r| r.expect("Failed to parse integer"))
                                .collect::<Vec<usize>>(),
                            Some(output_path_str.unwrap().clone()),
                        ),

                        // 2
                        o_tag_pos => {
                            // Split vecs at "-o" tag
                            let (before_o, _) = args.split_at(o_tag_pos);

                            // Split vecs one after "-o" tag
                            let (_, one_after_o) = args.split_at(o_tag_pos + 2);

                            // Join those arrays
                            let joined = before_o
                                .iter()
                                .chain(one_after_o.iter())
                                .skip(2)
                                .map(|s| s.parse::<usize>())
                                .map(|r| r.expect("Failed to parse integer"))
                                .collect::<Vec<usize>>();

                            (joined, Some(output_path_str.unwrap().clone()))
                        }
                    };
                    if args[1].as_str().eq("delete") {
                        Ok(TodoCommand::Delete { list, file })
                    } else {
                        Ok(TodoCommand::Done { list, file })
                    }
                }
            }
            "edit" => {
                // -o  tag should be in back of the command
                let o_tag_pos = args.iter().position(|x| x.as_str().eq("-o"));
                let output_path_str = o_tag_pos
                    .and_then(|i| args.get(i + 1))
                    .map(|s| s.to_owned());

                let (index, string) = (
                    args.get(3)
                        .expect("Index expected")
                        .parse::<usize>()
                        .expect("Cannot parse index to integer"),
                    args.get(4)
                        .map(|x| x.clone())
                        .expect("Please provide string to change"),
                );

                if let Some(path) = o_tag_pos.and(output_path_str) {
                    Ok(TodoCommand::Edit {
                        index,
                        string,
                        file: Some(path),
                    })
                } else {
                    Err("Tag -o specified but no path provided".to_owned())
                }
            }
            "clear" => {
                let o_tag_pos = args.iter().position(|x| x.as_str().eq("-o"));
                let output_path_str = o_tag_pos.and_then(|i| args.get(i + 1));

                Ok(TodoCommand::Clear {
                    file: Some(output_path_str.unwrap().clone()),
                })
            }
            "read" => {
                let o_tag_pos = args.iter().position(|x| x.as_str().eq("-o"));
                let output_path_str = o_tag_pos.and_then(|i| args.get(i + 1));
                let include_done = args.iter().any(|s| s.as_str().eq("-d"));

                if let None = o_tag_pos {
                    return Ok(TodoCommand::Read {
                        file: None,
                        include_done,
                    });
                }

                if let None = o_tag_pos.and(output_path_str) {
                    Err("Found -o tag but no file was specified".to_owned())
                } else {
                    Ok(TodoCommand::Read {
                        file: Some(output_path_str.unwrap().clone()),
                        include_done,
                    })
                }
            }
            _ => Err("Unknown command".to_owned()),
        }
    }
}
