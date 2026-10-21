+++
title = "Cats Theory Primer: Ground Zero"
date = 2025-09-18
description = "An accessible introduction to category theory from the perspective of a tech product person"
card = "cats-in-space.jpg"
banner = "cats-in-space.jpg"
+++

### Introduction

Coming from a background in tech product development rather than pure mathematics or coding, I've recently begun exploring category theory. While I'm familiar with the work of J.G. Bennett and have been exposed to mathematical and programming concepts, I'm essentially starting from the ground up when it comes to formal category theory. This exposition shares what I've learned so far, presented in a way that hopefully makes these concepts accessible to others in similar positions.

### Foundations: Beyond Set Theory

Traditional mathematics typically builds upon set theory as its foundation. However, category theory offers something different—and arguably more powerful. While sets and categories are fundamentally different constructs, category theory provides greater explanatory power and can integrate vast swaths of the mathematical landscape into a unified framework.

Where set theory focuses on membership and elements, category theory emphasizes relationships and transformations. This shift in perspective proves remarkably fruitful for understanding mathematical structures and their interconnections.

### The Basic Building Blocks

#### Objects and Morphisms

If you're familiar with graph theory or network diagrams, you already understand the visual metaphor: nodes connected by edges. In category theory, these become:

- **Objects**: The nodes or dots in our diagram (though they're more abstract than just points)
- **Morphisms**: The arrows or edges that connect objects (also called arrows or maps)

A category consists of these objects and morphisms, bound together by three fundamental laws that ensure mathematical coherence.

#### The Three Laws

##### 1. Identity
Every object must have an identity morphism—an arrow that loops back to itself. This self-referential morphism leaves the object unchanged, much like multiplying by 1 or adding 0 in arithmetic. Visually, this appears as a loop starting and ending at the same object.

##### 2. Associativity
When we have multiple morphisms connecting objects, the order in which we group them doesn't matter. If we have morphisms f, g, and h that can be composed, then (f ∘ g) ∘ h = f ∘ (g ∘ h). This property ensures consistency in how we navigate through our categorical structures.

##### 3. Composition
Perhaps the most powerful law: whenever we have a path from object A to B, and another from B to C, there must exist a direct morphism from A to C that represents their composition. We say this diagram "commutes"—meaning all paths between the same endpoints are equivalent. This creates a rich network of relationships where indirect connections imply direct ones.

### Types of Morphisms

Morphisms come in various flavors, each capturing different mathematical relationships:

- **Monomorphism**: Injective or "one-to-one" morphisms that preserve distinctness (constructive in nature)
- **Epimorphism**: Surjective or "onto" morphisms that cover the target completely (deconstructive in nature)
- **Isomorphism**: Morphisms with perfect two-way correspondence—essentially saying two objects are "the same" from the category's perspective
- **Homomorphism**: Structure-preserving transformations that maintain the essential properties while mapping between objects

### Higher-Order Structures

#### Functors: Morphisms Between Categories

Just as morphisms connect objects within a category, functors connect entire categories to each other. A functor maps:
- Objects in one category to objects in another
- Morphisms in one category to morphisms in another
- While preserving the categorical structure (identity and composition)

This creates a second-order pattern where categories themselves become objects in a higher-level view, connected by functors as their morphisms.

#### The Hierarchy Continues

This pattern of abstraction continues indefinitely in higher category theory:
- **2-categories**: Categories where morphisms between morphisms (called 2-morphisms) exist
- **3-categories**: Adding another level of morphisms between 2-morphisms
- **n-categories**: The pattern continues to arbitrary levels of abstraction

Each level maintains the same fundamental pattern of objects and arrows, creating a fractal-like structure of mathematical relationships.

### Further Exploration

This primer barely scratches the surface. Key topics for deeper investigation include:
- Natural transformations
- Limits and colimits
- Topos theory
- Monoidal categories
- Applied category theory
