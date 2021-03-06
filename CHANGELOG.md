# Changelog

## [0.10.0] - 2021-03-07
## Added
- Zero trait to transfer functions
- Addition between scalar and transfer function
- Polynomial exponentiation
- Relative degree of a transfer function
- Discretization of a continuous time transfer function now can return a discrete time transfer function
- Tustin discretization with pre-warping frequency
- Module for rational functions
## Changed
- Updated dependencies to latest available versions
- Oldest supported rustc version has been increased to 1.44
- Changed random testing library from quickcheck to proptest
### API Changes
- `utils` module has been split into different public modules: `complex`, `enums`
- External dependencies are now re-exported from this library
## Fixed
- Improved the formatting of polynomials and transfer functions

## [0.9.0] - 2020-12-05
## Added
- Evaluation of polynomial ratios that avoids overflows
## Changed
- Add and example for a car suspension system
- Changed division method for complex numbers
- Decoupled a plot (polar, Bode, root locus) from the iterator that supplies points
- Removed limitation in the creation of transfer function with zero numerator or denominator
### API Changes
- Implemented a custom Error type
- Refactored Polar struct
- Refactored Bode struct
- Removed DiscreteTime trait
## Fixed
- Changed complex number division method

## [0.8.0] - 2020-08-30
## Added
- Documentation of the specification of the library
- Evaluation of polynomial and transfer functions now take reference values too
- Add polynomial creation from iterators
- AsRef trait to polynomial for reference-to-reference conversion into a slice
- decibel unit of measurement
- Polynomial round off to zero
- Conversion from TfGen to SSGen using controllability canonical form
- Methods to check transfer function stability
- Auto implementation of PartialEq trait for SSGen
- Auto implementation Clone trait for TfGen and SSGen
- Methods to check transfer functions stability
## Changed
- Improvements on polynomial roots finders and increment of related tests
- General source code linting
- Use IntoIterator trait as interface in methods that require an iterator
- Evaluation of polynomials and transfer functions now can take references
- Polynomial numeric type no longer requires Copy trait
### API Changes
- Changed iterator names (C-ITER-TY API guidelines)
- Changed linear system solvers names (C-ITER-TY API guidelines)
- Changed linear system evolution iterators names (C-ITER-TY API guidelines)
- Changed arma iterators names (C-ITER-TY API guidelines)
- Renamed Decibel trait to ToDecibel
- Moved Discretization to top module
- Removed Eval trait
- Removed TryFrom trait from TfGen
- Moved the implementation of the polynomial matrices to its own module
## Fixed
- Subtraction between a real number and a polynomial
- Derivation of zero degree polynomial
- Conversion from transfer function to state space representation
- Derivation of zero degree polynomials

## [0.7.0] - 2020-02-08
## Added
- Implementation of transfer function arithmetic operations
- Implementation of transfer function feedback
- Check for system stability
- Common input signals
- Transfer function static gain
- Transfer function positive and negative feedback
- Transfer function sensitivity functions
- Transfer function delay
- Equilibrium for discrete systems
- Root locus plot
- Controllability and observability matrices
- Autoregressive moving average (ARMA) model of discrete transfer function
- Polynomial division
- Polynomial root finding using iterative Aberth-Ehrlich method.
- Polynomial multiplication using fast fourier transform
## Changed
- Split type for continuous and discrete transfer functions
- Split type for continuous and discrete state space representation

## [0.6.0] - 2019-11-18
## Added
- Increased the quantity of tests and documentation tests
- Polynomial derivation and integration
## Changed
- Generalization of polynomials
- Generalization of PID
- Generalization of discretization methods
- Generalization of transfer functions
- Generalization of linear systems
- Generalization of polynomial matrices
- Generalization of transfer function matrices
- Generalization of units of measurement
- Generalization of discrete transfer functions
- Generalization of Bode and polar plots
- Generalization of discrete linear systems
- Generalization of linear system solvers
- The degree of a polynomial now returns an Option, which is None for zero polynomial
- Companion matrix is None for zero degree polynomial
## Fixed
- Error in the calculation of ideal PID transfer function
- Error in the calculation of 2x2 matrix eigenvalues
- Error in state space Tustin discretization

## [0.5.0] - 2019-09-08
### Added
- Discretization of transfer functions
- Units of measurement
### Changed
- Use typed unit of measurement instead of primitive types

## [0.4.1] - 2019-09-01
### Added
- Documentation links for discrete system.
### Changed
- Applied clippy pedantic suggestions.

## [0.4.0] - 2019-08-26
### Added
- Implemented Runge-Kutta solver of order 4.
- Discrete linear systems time evolution.
- Discretization of continuous linear system.
- Allow to pass closures as input for the time evolution of a system.
- Example for system discretization.
### Changed
- Improve efficiency using LU decomposition to solve implicit system.
### Fixed
- Corrected the transformation from transfer function to state-space form.

## [0.3.0] - 2019-08-05
### Added
- Radau implicit ordinary differential equations solver.
- Crate and module documentation.
- Example for stiff system.
### Fixed
- Calculation time inside ordinary differential equations solvers.

## [0.2.1] - 2019-08-01
### Changed
- Time evolution method requires a function as input.
- Add tolerance to the adaptive step size Runge-Kutta solver (rkf45) as parameter.
- Use time limit for the rkf45 solver.
### Fixed
- The output of the system is calculate with the time at the end of the step in the Runge-Kutta solvers.

## [0.2.0] - 2019-07-27
### Added
- First release with initial development
