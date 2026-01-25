# Paycheck-Utils
---

(a work in progress)

- This project is an exercise in developing a personal Rust utility library with useful functions for managing paycheck income.
- It is the final assignment for the course "[Rust Fundamentals](https://www.coursera.org/learn/rust-fundamentals)" taught by Alfredo Deza representing Duke University on the Coursera learning platform.
- The assignment instructions are as follows:

> ### External Lab: Create a Rust library

In this lab, you will create a library in Rust to reinforce the concepts covered in the last lesson. You can choose one of the following library ideas, or you are free to create one on your own. You will add code to the lib.rs file, document your code, and utilize tools like Makefile and Cargo. The end result will be a GitHub repository containing the complete code for your chosen library.

Learning Objectives:

    - Gain experience in creating a library and organizing code in Rust.

    - Practice documenting code using comments and Rust's documentation conventions.

    - Understand how to use a Makefile to automate build tasks and simplify compilation.

    - Explore public and private modules in Rust and define appropriate access levels for functions.

Steps:

    1. Create a new repository in your account for your Rust library project. 
    (You can also use the Rust template repository to quickly generate the scaffolding for your project in your own account.)

    2. Use the example code used for this week as a starting point

    3. Use one of the library ideas below or implement one from your own

Bonus: Try publishing your documented library to crates.io so that you can share it with others as a crate.

---

> ### Project Purpose

I chose to create a library that was personally relevant and useful to me -- specifically, a library that contains utility functions for estimating paycheck withholdings and net income in order to compare against a given set of living expenses. The idea is similar to the "Sample Paycheck" tool found in the [Paycom](https://www.paycom.com/software/employee-self-service/) employee portal, but aimed at having a little more flexibility.

---

#### Example Calculations:

- Given a list of monthly expenses and a list of elected payroll deductions, if I work [n] hours per week at a rate of [m] dollars per hour, would my net monthly income be enough to cover the expenses with at least $500 left over.

---

####  * A Note on Copilot usage *

Copilot is initially paused for a duration of 30 mins when I first begin a task to allow me time to think through the solution on my own and try to implement the ideas and features I have in mind. If I cannot seem to come up with a solution in 30 mins, I begin to allow Copilot to offer suggestions and then determine if it fits what I am trying to accomplish. If I do not understand the code offered by Copilot, I ask for an explanation of what it's doing. Copilot does not always know what I am trying to do. For example, the modules and key words I have used in this library make Copilot think I am trying to create a library for filing taxes at tax season, but the library is intended to help calculate payroll taxes, deductions, and income/expenses -- not federal income taxes, deductions and income/expenses as filed during tax season.

> ### See the Documentation