// use declarative::declarative;
// use derive_macro::Builder;

// #[derive(Debug, Builder)]
// struct Command<'a> {
//   executable: String,
//   args: Vec<String>,
//   env: Vec<String>,
//   current_dir: &'a str,
// }

#[derive(Debug)]
struct DeriveInput {
  data: Data,
}

#[derive(Debug)]
enum Data {
  Struct(DataStruct),
}

#[derive(Debug)]
struct DataStruct {
  fields: Fields,
}

#[derive(Debug)]
enum Fields {
  Named(FieldsNamed),
}

#[derive(Debug)]
struct FieldsNamed {
  named: String,
  size: i8,
}

#[derive(Debug)]
enum IP {
  IpV4(i8, i8, i8, i8),
}

fn main() {
  // let _docker_cli: Command = Command {
  //   executable: String::from("docker container run postgres"),
  //   args: vec![String::from("-d"), String::from("--rm")],
  //   env: vec![String::from("POSTGRES_PASSWORD=mysecretpassword")],
  //   current_dir: "./",
  // };

  // println!("{:#?}", docker_cli);
  // declarative::run();

  let data = DeriveInput {
    data: Data::Struct(DataStruct {
      fields: Fields::Named(FieldsNamed {
        named: String::from("this is name"),
        size: 10,
      }),
    }),
  };

  // let size = match data {
  //   DeriveInput => match DeriveInput.data {
  //     Data::Struct(data_struct) => match data_struct {
  //       DataStruct => match DataStruct.fields {
  //         Fields::Named(fields_named) => match fields_named {
  //           FieldsNamed => FieldsNamed.size,
  //           _ => panic!("Lv 5"),
  //         },
  //         _ => panic!("Lv 4"),
  //       },
  //       _ => panic!("Lv 3"),
  //     },
  //     _ => panic!("Lv2"),
  //   },
  //   _ => panic!("Lv1"),
  // };

  // println!("Size ne: {}", size);

  let ip = IP::IpV4(127, 0, 0, 1);
  println!("what ne {:#?}", ip);
  let what = if let IP::IpV4(i, a, b, c) = ip {
    (i, a, b, c)
  } else {
    panic!("aaa");
  };

  println!("what ne {:#?}", what);

  let ok = if let DeriveInput {
    data:
      Data::Struct(DataStruct {
        fields: Fields::Named(FieldsNamed { ref named, .. }),
      }),
    ..
  } = data
  {
    named
  } else {
    panic!("Lv1")
  };

  println!("ok ne {:#?}", ok);

  let f = Fields::Named(FieldsNamed {
    size: 20,
    named: String::from("hihi"),
  });


  let ff = if let Fields::Named(FieldsNamed) = f {
    FieldsNamed
  } else {
    panic!("fff ne");
  }; 

  println!("what {:#?}", ff);
}
