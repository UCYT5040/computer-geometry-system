# Methods

Methods are the various properties and functions that can be called on objects in the CGS.

## Method Files

```jsonc
{
    "properties": [
        // Various properties
    ],
    "relations": [
        // Various relations
    ]
}
```

## Properties

Properties can be assigned to objects within the CGS.

```jsonc
{
    "name": "nameOfProperty",
}
```

You can reference properties at any time using their name. For child properties, use a period (.) to access them.

### Types

Properties can be of various types, such as:

- `defined`: Similar to `boolean`. If this property is present, it is considered `true`, otherwise it is considered `false`.
- `number`: An algebraic value.
- `point`: A point in 2D space.
- `segment`: A segment in 2D space. Can also be treated as `line`.
- `line`: A line in 2D space.
- `boolean`: A boolean value, either `true` or `false`.
- `list`: A list of values (set `itemType`)

#### Special List Properties

Consider a polygon.

It has a property `sides`, which is a list of segments.

But for some operations, we only need to know the number of sides, not the actual segments.

Lists provide a special `length` property. Simply set the `length` key to the name of the property that will hold the length of the list.

This property does not depend on list content (that is, you can set the length without knowing the data).

```
{
    "name": "sides",
    "type": "list",
    "itemType": "segment",
    "length": "numberOfSides"
}
```

## Relations

Relations are used to, well, relate properties to each other.

### Types

#### Primitive relations

Primitive relations are simple relations that directly relate properties to each other. They should be used whenever possible as they are more efficient to evaluate.

- `equal`: Two-way equality between `propertyA` and `propertyB`.
- `implies`: One-way implication from `propertyA` to `propertyB`. (that is, A => B, but not necessarily B => A)
- `notEqual`: Two-way inequality between `propertyA` and `propertyB`.
- `notImplies`: One-way non-implication from `propertyA` to `propertyB`. (that is, A => !B, but not necessarily B => !A)

#### Complex relations

- `equation`: A CAS equation relating properties. Often preferred over expressions because it can be solved for any property.
- `expression`: A single CAS expression that defines a property in terms of other properties.

Complex relations are handled by CAS.

For equations, define the properties and what variable to use. Then provide a CAS equation that relates the properties.

```jsonc
{
    "type": "equation",
    "properties": {
        "a": "propertyA",
        "b": "propertyB",
        "c": "propertyC"
    },
    "equation": "a + b = 10c"
}
```

For expressions, define the inputs and what variable to use. Then provide a CAS expression that defines the output property in terms of the input properties.

```jsonc
{
    "type": "expression",
    "inputs": {
        "a": "propertyA",
        "b": "propertyB"
    },
    "output": "propertyC",
    "expression": "a^2 + b^2"
}
```
