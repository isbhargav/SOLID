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

```
// BAD design (srp violated)
class Task{
  setTask()
  markDone()
  status()
  downloadFile()
  parseFile()
  persistData()
}

// GOOD design (srp followed)
class Task{
  setTask()
  markDone()
  status()
}

class TaskDownloader(){
  downloadFile()
  parseFile()
  persistData()
}



```
## Open-Closed Principle 

Your software artifactes should be open for extension, but closed for modification. You should be able to add new functionalites to existing entities without having to intorduce the risk of changing existing behaviours. A good way to spot that you are violating this principle is to notice when you are using chain of if.. else.. Statement, this is usually a sign that your function is not open to any extension as you would have to make modification at this place if you were to add new functionality to this class. 
 

Example:

Class Inheritance, Hooks (wrap other hooks inside), children prop.

```
// BAD design (ocp violated)
for shape in SHAPES:
  if shape == "square":
    area_square(shape);
  else if shape == "circle":
    area_circle(shape)
  else if shape == "recatangle":
    area_rectangle(shape)

// GOOD design (ocp followed)
for shape in SHAPES:
  shape.area()

```

## Liskov Substitution

Liskov Substitution defines that objects of a  superclass shall be replaceable with objects of its subclasses without  breaking the application. That requires the objects of your subclasses  to behave in the same way as the objects of your superclass. It extends the Open/Closed Principle by focusing on the behavior of a superclass and its subtypes. As I will show you in this article, this is at least as important but harder to validate that the structural requirements of the Open/Closed Principle.
You can achive this in languages like OO by extendning from the base class(shown in example) or Introducing shared interface (like we did in dependency inversion example).

Example:

React Query, SWR (the extend making http request functionality to make use of caching ).

```
// BAD
Office{
  coffeMachine: BasicCoffeMachine
  numberOfEmployees: int
  prepareCofeeForAllEmployee(){
    return map((0..numberOfEmployees),coffeMachine.make_coffe())
  }
}

PremiumCoffeMachine{
  make_coffe()
}

Office(new PremiumCoffeMachine(), 12); // XXX WE CANNOT DO THIS BEACUSE PremiumCoffeMachine IS TOTALLY DIFFERENT CLASS EVEN IF IT SHARES SAME METHOD

// GOOD
Office{
    coffeMachine: BasicCoffeMachine
      numberOfEmployees: int
        prepareCofeeForAllEmployee(){
              return map((0..numberOfEmployees),coffeMachine.make_coffe())
                }
}

PremiumCoffeMachine extends BasicCoffeMachine{
    make_coffe()
}

Office(new PremiumCoffeMachine(), 12); // WE CAN DO THIS NOW

```

## Interface Segregation

A client should never be forced to implement an interface that it doesn’t use, or clients shouldn’t be forced to depend on methods they do not use. Similar to the Single Responsibility Principle, the goal of the Interface Segregation Principle is to reduce the side effects and frequency of required changes by splitting the software into multiple, independent parts. This is only achievable if you define your interfaces so that they fit a specific client or task. This princple state that you should create more fine grain interfaces instead of genric Interface.


Example:

React and ReactDOM,

```
// BAD
IWorker{
  StartWork()
  StopWork()
  getChargeStatus()
  reCharge()
  lunckBreak()
  takeWashroomBreak()
}
HumanWorker implements IWorker{
  StartWork()
  StopWork()
  getChargeStatus() -> NOP
  reCharge()  -> NOP
  lunckBreak()
  takeWashroomBreak()
}

RobotWorker implements IWorker{
  StartWork()
  StopWork()
  getChargeStatus()
  reCharge()
  lunckBreak() -> NOP
  takeWashroomBreak() -> NOP
}


// GOOD
IWorker{
  StartWork()
  StopWork()
}
IHumane{
  lunckBreak()
  takeWashroomBreak()
}
IRobot{
  getChargeStatus()
  reCharge()
}

HumanWorker implements IWorker,Human{
  StartWork()
  StopWork()
  lunckBreak()
  takeWashroomBreak()
}

RobotWorker implements IWorker,Robot{
  StartWork()
  StopWork()
  getChargeStatus()
  reCharge()
}
```

## Dependency Inversion

Entities must depend on abstractions, not on concretions. It states that the high-level module must not depend on the low-level module, but they should depend on abstractions.

Exmaple:

React Context

```
// BAD
duck {
 walk()
 quack()
}

robotduck extends duck {
  walk(),
  quakc()
}

simulate(duck : duck) { }
simulate(robotduck : robotduck){}


// GOOD 
IDuck{
  walk(),
  quack()
}
duck implements IDuck{
 walk()
 quack()
}

robotduck implements IDuck{
  walk(),
  quakc()
}

simulate(duck : iduck)
{
  // simulate the duck no matter what type
}


```
