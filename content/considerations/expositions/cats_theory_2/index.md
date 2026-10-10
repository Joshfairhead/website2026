+++
title = "Cats Theory Primer: Reasoning and Revisions"
date = 2025-09-19
description = "Addressing critiques and diving deeper into the baics of category theory through examples and definitions"
card = "cats-in-space.jpg"
banner = "cats-in-space.jpg"
+++

### Reflection on Feedback

Sharing my [last post](../cats_theory_1/) the critique was blunt: "nothing here that you can't find in an introductory book on Category Theory". True - and you would probabaly find a whole lot more there too! 

The purpose was and is:
1) learn in public
2) share where I'm at
3) test my understanding

Two books were recommended requiring almost no background:
1) [Computational Category Theory](https://www.cs.man.ac.uk/~david/categories/book/book.pdf)
2) [Category Theory for Programmers](https://unglueit-files.s3.amazonaws.com/ebf/e90890f0a6ea420c9825657d6f3a851d.pdf)

The constructive elements of said critique asked some questions; namely whats and object and morphism? How are they defined? Provide an example of these. What is a category, provide an example, how do you split categories into different types? So here is another attempt at answering:

### What is an Object?

An object is a conceptual placeholder. You might consider it a point, which would indeed be an object but it's more general than that so better thought of as a conceptual element within a space. In this example the point in topological space is an object.

### What is a Morphism?

A morphism is a conceptual mapping. You might consider it a line or arrow, which would indeed be a morphism but technically they are more general than that, so probably better defined as a mapping between objects. In this example a line in topological space is a morphism.

### What is a Category?

A category is a conceptual space. At this point, we run into difficulty relying on the topology analogy. If we were to keep going with that general path of abstraction we would say its a plane because that's the next topological level, but I'm pretty sure the theorists would be horrified!

Let's wrestle with this alligator:

Based on the previous definition a category requires identity, association and composition. I'm going to go out on a limb and describe a category as form of relational or third order logic. 

A category is an abstract conceptual space with concrete instantiations; a map is an abstraction of the territory, A menu is an abstraction of the meal. A category is an abstraction of any structured grouping of objects and morphisms provided there is identity, associativity and composition.

We need to cover these three rules in order to define a category. Then our decision of whether something is a category or not is based on the morphisms we observe. 

#### Identity
Identity is a morphism from an object to itself that is essentially a non-operation because it returns no changes. 

In programming languages where there is a type system, what happens is that the morphism takes an object with a generic type and paramater, returning the same type and paramater. Just replace "type" with "object" or "class" in other languages.

``` rust
pub fn identity<T>(x: T) -> T {
    x
}
```

This may sound practically useless but it's not. Think of it as a neutral gear, you need it as a between state for changing gears. You don't exactly drive in neutral but it allows you to flexibly chain sequences together into a desired result. 


### Composition

Composition is the assembly of morphisms into a new morphism. If we have action A and action B, composition is about unifying both morphisms into action C. It is always a binary operation that turns two morphisms into one, but they compositions can be chained. Because of association these chains can be arbitrary length. The order DOES matter which is to say that:

``` 
a ∘ b 
```
and
``` 
b ∘ a
```
are not the same. 


#### Association

Association is a property which means that in a series of operations, the grouping of objects DOES NOT matter, the operations do. Written like this:
``` (a ∘ b) ∘ c = a ∘ (b ∘ c) ```

Taking a jar of coins for example, some operations will be associative and others will not be. If we take the above and fill in the formula with different values and operations we will get different results:

``` Associative
Left grouping: (5 + 1) + 4 = 10 
Right grouping: 5 + (4 +1 ) = 10
```

``` Non-associative
Left grouping: (5 - 1) - 4 = 0
Right grouping: 5 - (1 - 4) = 8
```


### Lets make it personal

We have the object of ourself, and morphisms of our actions and interactions. 

An identity morphism is quite literally a relationship to self. But since it must be the unchanging self, it would indicate the neutral observer of some variety.

Association would be when morphisms (actions and interactions) can be grouped without consequence: 

```
Left side: ((sleep ∘ brush teeth) ∘ eat breakfast)
First eat breakfast, then brush teeth, then sleep

Right side: (sleep ∘ (brush teeth ∘ eat breakfast))
First eat breakfast, then brush teeth, then sleep
```

Lastly there's composition. Lets take the above. The associative grouping doesn't matter as we have seen but the ordering does; the result should always be sleeping with clean teeth after brekfast. Change the order and your teeth are no longer clean when you go to sleep!

Thats it. I suppose thats the category of life. 

All this is not that useful, it may clarify some things but besides that you're mind probably not blown. I mean who cares about this mundane stuff right? I dont. 

But perhaps if we do have a relationship with ourselves we can learn to navigate the world thats not just floating on the breeze as victims of hazard and uncertainty? 

Let's put it another way; becoming the true self means understanding transformation at a deep level. Category theory and mathmatics gives us an annoyingly precise language for describing how to achieve that if we understand how memetic patters commute and such. 

How is it that I came to be how I am? and can I be otherwise? And if I can be otherwise, how may I choose the shapes and forms that I may become? Am I choosing or is inertia choosing? Can I break free of inertia, if so how? 

### Free categories vs non-free categories.

There are various classes of category but the fundemental polarity seems to be free and non-free, which is to say that there are abstract and concrete categories.

A free category is a pure abstraction or 'universal construction' that has no rules or restrictions imposed on it other than whats necessitated by the axiomatic definitions of a category; those of identity, associativity and composition. A non-free category is one presented with additional constraints. 

Lets make that a little more lucid by giving an example. 

The category of a 'free society' would be a society that engages only through the basic rules of social connection without norm, laws or special relationships. 

For example: 

```
Alice --friends--> Bob --dating--> Carol
``` 

- Alice (identity: being herself)
- Bob (identity: being himself)
- Carol (identity: being herself)
- "Alice is friends with Bob"
- "Bob is dating Carol"
- "Alice knows Carol through Bob" (forced by social transitivity)

The category of a 'non-free society' adds rules such as you must be friends with your partners friends. So Alice not only knows Carol through Bob, but is forced to be her friend. 

Some other examples might be family relations or corporate hierarchies because they enforce extra rules; if A is parent of B and B is parent of C then A is grandparent of C. 

# Fin
