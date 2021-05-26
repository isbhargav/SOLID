# SOLID
 SOLID Principles design practice

# S.O.L.I.D Principle

SOLID is an acronym for the first five object-oriented design (OOD) principles by Robert C. Martin. These principles establish practices that lend to developing software with considerations for maintaining and extending as the project grows. Adopting these practices can also contribute to avoiding code smells, refactoring code, and Agile or Adaptive software development.

SOLID stands for:

S - Single Responsiblity

O - Open-Close

L - Liskov substitution

I - Interface segration 

D - Dependency Inversion



## Single Responsibility

Every module or class should have single responsiblity over single part of functionality. Single functionality is a subjective defination and so it can very different based on different context. A good way to think about single resonsiblity is to think in terms of **change**. Think like what should be the reason for which you might need to change your code or logic inside your module or class, this should be enough to identify whether your class or module is following single responsibility principle. *Your Compoenent should change for only one reason. - Robert C. Martin* 

Examples: 

React Compoenent - Display state of application (Should change only when you want to change how it looks) , Hooks (useState mange local state of component, useEffect manage sideeffects ), API Services (fetch state from backend of the application ), Reducers (update state based on other states).

## Open-Closed Principle 

Your software artifactes should be open for extension, but closed for modification. You should be able to add new functionalites to existing entities without having to intorduce the risk of changing existing behaviours. 

Example:

Class Inheritance, Hooks (wrap other hooks inside), children prop.

## Liskov Substitution

Liskov Substitution defines that objects of a  superclass shall be replaceable with objects of its subclasses without  breaking the application. That requires the objects of your subclasses  to behave in the same way as the objects of your superclass.

Example:

React Query, SWR (the extend making http request functionality to make use of caching ).

## Interface Segregation

A client should never be forced to implement an interface that it doesn’t use, or clients shouldn’t be forced to depend on methods they do not use.

Example:

React and ReactDOM,

## Dependency Inversion

Entities must depend on abstractions, not on concretions. It states that the high-level module must not depend on the low-level module, but they should depend on abstractions.

Exmaple:

React Context
