fn main() {
      let student1= Student{
            name: String::from("Carl Tumbo"),
            student_id: 2042,
            student_email: String::from("carl2042@school.com"),
            department: String::from(Botany),
      };
      //Creating an instance of a struct from another instance.
      //The String data is moved from one instance to another, hence String values from student1 is moved to student2.
      let student2= Student{
            name: String::from("Dan Marshall"),
            student_id: 2043,
            ..student1
      };

}
struct Student {
    name: String,
    student_id: i32,
    student_email: String,
    department: String
}
fn add_student(name: String,student_id: i32,student_email: String) -> Student{
    Student{
        name,
        student_id,
        student_email,
    }
}