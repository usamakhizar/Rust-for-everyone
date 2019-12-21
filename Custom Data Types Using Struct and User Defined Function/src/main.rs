//1-Structure is created with one String datatype
      #[derive(Debug)] //I Added this adove the struct definition to have rust extend the debug trait for printing its complete instance 
      struct Students {
        student_name : String,
        student_id : u32  ,
        cgpa : f64,      
    }
   
fn main() {

//2A-An instance of the above defined struct
    let _stu1 = Students {
    student_name: "Student1".to_string(), //used to.string() method to assign string to struct string member
    student_id: 1234,
    cgpa:3.9
    };

//2B-Printing complete instance  
    println!("\nStudent1 :{:?}", _stu1);

//2C-Printing instance by calling its fields    
    println!("\nStudentName: {}\nStudentId: {}\nStudentCGPA: {}",_stu1.student_name,_stu1.student_id,_stu1.cgpa);

//2D-Creating another instance by using fields of first instance
    let _stu2 = Students{
        student_name : _stu1.student_name,
        student_id : _stu1.student_id,
        cgpa : _stu1.cgpa
    };
//printing _stu2
    println!("\nStudent2 :{:?}", _stu2);

//4-calling user defined function and printing instance returned by the user defined function.

    println!("\nStudent3 :{:?}",process("Student3".to_string(),7890,3.7));


    println!("\n------------Program End!------------\n");
   }

//3-Defining user defined function with arguments and returned an instance of Struct Students
    fn process (_name:String,_id:u32,_cgpa:f64) -> Students{
        let _stu3 = Students{
        
        student_name : _name,
        student_id : _id,
        cgpa :_cgpa
        };
        return _stu3;
    }
