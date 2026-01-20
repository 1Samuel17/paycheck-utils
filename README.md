# Tax-Tools
---

(a work in progress)

- This project is an exercise in developing a personal Rust utility library with useful functions for managing income taxes.
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

    1. Create a new repository in your account for your Rust library project. You can also use the Rust template repository to quickly generate the scaffolding for your project in your own account.

    2. Use the example code used for this week as a starting point

    3. Use one of the library ideas below or implement one from your own

    Bonus: Try publishing your documented library to crates.io so that you can share it with others as a crate.

---

> ### Project Purpose

I chose to create a library that was personally relevant and useful to me -- specifically, a library that contains utility functions for calculating net income, necessary hours, elected deductions, and standard tax withholding given a hypothetical set of expenses and a hypothetical hourly wage or salary scenario. The idea is pretty much like the "Sample Paycheck" tool found in the [Paycom](https://www.paycom.com/software/employee-self-service/) employee portal, but aimed at having more functionality and customization.

---

#### Example Calculations:

- Given a list of monthly expenses, how much should be earned hourly @ 40hrs/week (no overtime) to cover those expenses with at least $500 left for the month (not accounting for taxes)

- Given a list of monthly expenses, how much should be earned hourly @ 40hrs/week (no overtime) to cover those expenses with at least $500 left for the month (accounting for standard tax withholding)

- Given a gross annual salary and a list of elected deductions, what would the net income be after taxes and deductions?

---

####  * A Note on Copilot usage *
```
Copilot is being used but not blindly. In some cases, depending on the task, Copilot is paused entirely to allow me time to try to come up with a solution on my own. But for the sake of speed, Copilot suggestions are reviewed and a determination is made as to whether the assistance is exactly relevant, needing revision, or completely not applicable to project goals. If Copilot suggests advanced code techniques that are beyond my current understanding of Rust, I DO NOT implement it! Instead I make a note of the suggestion to do research at a later time to grow in my knowledge and proficiency of the language.

```


> ### See the Documentation