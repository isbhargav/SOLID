//encapsulation unit in Rust is module, so just make a static in the module your struct is in( eg: this file is Employee.rc)
#[allow(dead_code)]
pub const LEAVES_ALLOWED: i32 = 73;

#[derive(Debug)]
pub struct Employee {
    emp_id: i64,
    name: String,
    monthly_salary: f64,
    manager: String,
    leaves_taken: i32,
    years_in_org: i32,
    leaves_left_previously: Vec<i32>,
}

impl Employee {
    pub fn new(
        emp_id: i64,
        name: String,
        monthly_salary: f64,
        manager: String,
        leaves_taken: i32,
        years_in_org: i32,
        leaves_left_previously: Vec<i32>,
    ) -> Self {
        return Self {
            emp_id,
            name,
            monthly_salary,
            manager,
            leaves_taken,
            years_in_org,
            leaves_left_previously,
        };
    }
}

pub trait SerializeToHtml {
    fn to_html(&self) -> String;
}

pub struct EmployeeFormater<'a> {
    employee: &'a Employee,
}

impl<'a> EmployeeFormater<'a> {
    pub fn new(employee: &'a Employee) -> Self {
        return EmployeeFormater { employee };
    }
}

impl SerializeToHtml for EmployeeFormater<'_> {
    fn to_html(&self) -> String {
        let html_string = format!("<div><h1>Employee Info</h1><div id='emp{}'><span>{}</span><div class='left'><span>Leaves Left :</span> <span>Annual salary:</span> <span>Manager:</span> <span>Reimbursable leaves:</span> </div>",self.employee.emp_id,self.employee.name);
        return html_string;
    }
}
