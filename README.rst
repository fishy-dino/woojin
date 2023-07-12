woojin
======
.. image:: https://img.shields.io/crates/v/woojin
   :target: https://crates.io/crates/woojin
.. image:: https://img.shields.io/crates/dr/woojin
   :target: https://crates.io/crates/woojin
.. image:: https://img.shields.io/crates/l/woojin
   :target: https://crates.io/crates/woojin
.. image:: https://img.shields.io/github/languages/top/teamfishydino/woojin
   :target: https://github.com/teamfishydino/woojin
.. image:: https://img.shields.io/github/repo-size/teamfishydino/woojin
   :target: https://github.com/teamfishydino/woojin
.. image:: https://img.shields.io/github/last-commit/teamfishydino/woojin
   :target: https://github.com/teamfishydino/woojin

----

.. image:: https://img.shields.io/github/issues/teamfishydino/woojin
   :target: https://github.com/teamfishydino/woojin/issues
.. image:: https://img.shields.io/github/issues-closed/teamfishydino/woojin
   :target: https://github.com/teamfishydino/woojin
.. image:: https://img.shields.io/github/issues-pr/teamfishydino/woojin
   :target: https://github.com/teamfishydino/woojin
.. image:: https://img.shields.io/github/issues-pr-closed/teamfishydino/woojin
   :target: https://github.com/teamfishydino/woojin
.. image:: https://img.shields.io/badge/Support-Ukraine-FFD500?style=flat&labelColor=005BBB
  :alt: Support Ukraine
  :target: https://fishydino.rs/support-ukraine

Woojinlang interpreter written in Rust(Current v0.1.x).
=======================================================
The Woojinlang project is a project that was started as a joke to tease my friend. Do not use this language in your projects.

The **0.x.x** version is an unofficial version. Current grammar may not be compatible with the official version (>=1.x.x)

woojin is currently available in unofficial(beta) versions only, and there is no official release version. Therefore, it may not function perfectly and could have numerous bugs. We frequently receive reports of errors or bugs occurring in wooJin. We promptly address these reports and strive to minimize the errors present in wooJin. For example, the deadlock issue that occurred in v0.1.1 was resolved in the subsequent version, v0.1.2. Hence, we recommend using the latest version of woojin as it has fewer errors, a higher likelihood of proper functioning, and offers a wide range of features.

If you have discovered any issues (**bugs, etc.**) with woojin, please feel free to reach out to us through our **`GitHub issue page <https://github.com/teamfishydino/woojin/issues>`__** or via email at **teamfishydino@gmail.com**.
Additionally, if you have any questions about woojin or if you have ideas for additional features or improvements, we encourage you to utilize our **`GitHub discussion page <https://github.com/teamfishydino/woojin/discussions>`__**.
You can also find announcements and updates regarding woojin on our GitHub(**`Here! <https://github.com/teamfishydino/woojin/discussions/categories/announcements>`__**).

How To Install
--------------
* **Install Rust first** (if you have already installed the latest version of Rust on your computer, you may skip this process)
  https://www.rust-lang.org/tools/install
* **Download woojin with cargo**

.. code-block:: shell

   cargo install woojin --all-features

Example
-------
Here's an example for woojin v0.1.0.
Do you want more example? `Click Here! <https://github.com/teamfishydino/woojin-example/tree/main/example>`_

First, Create a file named main.wj.

.. code-block:: woojin

   println "My First Woojinlang Program!";
   let name = input "Hello, What is your Name? ";
   println "Hello, "+$name+". Nice to meet you!";
   yee 0;

And use the command below to run the woojin file

.. code-block:: shell

   woojin main.wj
