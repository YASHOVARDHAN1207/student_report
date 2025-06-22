use std::io;
use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

fn calculate_average(total_marks: f32, num_subjects: f32) -> f32 {
    total_marks / num_subjects
}

fn assign_grade(avg: f32) -> &'static str {
    if avg >= 90.0 {
        "A"
    } else if avg >= 75.0 {
        "B"
    } else if avg >= 60.0 {
        "C"
    } else {
        "D"
    }
}

fn main() {
    let mut name = String::new();
    let mut total_marks = String::new();
    let mut num_subjects = String::new();

    println!("Enter student name:");
    io::stdin().read_line(&mut name).unwrap();

    println!("Enter total marks:");
    io::stdin().read_line(&mut total_marks).unwrap();
    let total_marks: f32 = total_marks.trim().parse().unwrap();

    println!("Enter number of subjects:");
    io::stdin().read_line(&mut num_subjects).unwrap();
    let num_subjects: f32 = num_subjects.trim().parse().unwrap();

    let average = calculate_average(total_marks, num_subjects);
    let grade = assign_grade(average);

    println!("\nReport Card:");
    println!("Name: {}", name.trim());
    println!("Average: {:.2}", average);
    println!("Grade: {}", grade);

    generate_pdf(name.trim(), average, grade);
}

fn generate_pdf(name: &str, average: f32, grade: &str) {
    let (doc, page1, layer1) = PdfDocument::new("Report Card", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    // Change file path below if you use a different font name
    let font = doc
        .add_external_font(File::open("fonts/Arial.ttf").unwrap())
        .unwrap();

    current_layer.use_text(
        format!("Report Card for: {}", name),
        24.0,
        Mm(20.0),
        Mm(270.0),
        &font,
    );
    current_layer.use_text(
        format!("Average Marks: {:.2}", average),
        18.0,
        Mm(20.0),
        Mm(250.0),
        &font,
    );
    current_layer.use_text(
        format!("Grade: {}", grade),
        18.0,
        Mm(20.0),
        Mm(230.0),
        &font,
    );

    doc.save(&mut BufWriter::new(File::create("report_card.pdf").unwrap()))
        .unwrap();
}
