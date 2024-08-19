# CommonNarsese

[🔙Concept](./doc.md)

📍 Last updated: 【2024-04-10 15:39:37】

Based on the **Standard ASCII Lexicon** defined by [Narsese.rs](https://github.com/ARCJ137442/Narsese.rs)

## Syntax

A few basic principles:

- 📌 A form **only has** one expression method
- 📌 No constraints on specific content (atomic prefixes, compound connectors, predicative verbs)
- 📌 Task → Statement univocity

🔗 For specific content, see [the corresponding section of Narsese.rs](https://github.com/ARCJ137442/Narsese.rs/blob/main/README.en.md)

## Example Set

### Terms

#### Atomic Terms

Words: `word` `dash_separated` `slash-separated` `词语` `あ`

Placeholders: `_` `_placeholder`

Independent variables: `$independent_variable`

Dependent variables: `#dependent_variable`

Query variables: `?query_variable`

Intervals: `+123` `+137`

Operators: `^operator` `^go-to`

#### Compound Terms

Extensional set: `{term1, $i_term2, #d_term3, ?q_term4, ^o_term5}`

Intensional set: `[term1, $i_term2, #d_term3, ?q_term4, ^o_term5]`

Extensional intersection: `(&, term1, $i_term2, #d_term3, ?q_term4, ^o_term5)`

Intensional intersection: `(|, term1, $i_term2, #d_term3, ?q_term4, ^o_term5)`

Extensional difference: `(-, term1, term2)`

Intensional difference: `(~, term1, term2)`

Product: `(*, term1, $i_term2, #d_term3, ?q_term4, ^o_term5)`

Extensional image: `(/, term1, _, $i_term2, #d_term3, ?q_term4, ^o_term5)`

Intensional image: `(\, term1, _, $i_term2, #d_term3, ?q_term4, ^o_term5)`

Conjunction: `(&&, <term1 --> _placeholder>, <$i_term2 --> #d_term3>, <?q_term4 --> ^o_term5>)`

Disjunction: `(||, <term1 --> _placeholder>, <$i_term2 --> #d_term3>, <?q_term4 --> ^o_term5>)`

Negation: `(--, <term1 --> _placeholder>)`

Sequential conjunction: `(&/, <term1 --> _placeholder>, <$i_term2 --> #d_term3>, <?q_term4 --> ^o_term5>)`

Parallel conjunction: `(&|, <term1 --> _placeholder>, <$i_term2 --> #d_term3>, <?q_term4 --> ^o_term5>)`

#### Statements

##### Basic Copulas

Inheritance: `<term1 --> term2>`

Similarity: `<term1 <-> term2>`

Implication: `<term1 ==> term2>`

Equivalence: `<term1 <=> term2>`

##### Derived Copulas

Instance: `<term1 {-- term2>`

Attribute: `<term1 --] term2>`

Instance attribute: `<term1 {-] term2>`

Predictive implication: `<term1 =/> term2>`

Concurrent implication: `<term1 =|> term2>`

Retrospective implication: `<term1 =\> term2>`

Predictive equivalence: `<term1 </> term2>`

Concurrent equivalence: `<term1 <|> term2>`

### Stamps (not standalone)

Past: `:\:`

Present: `:|:`

Future: `:/:`

Fixed: `:!-10:` `:!+123:` `:!137:`

### Truth-values (not standalone)

Dual truth values: `%1.0; 0.9%`

Single truth value: `%1.0%`

Empty truth value: `%%` (or not present)

### Budget-values (not standalone)

Triple budget: `$0.5; 0.5; 0.5$`

Double budget: `$0.5; 0.5$`

Single budget: `$0.5$`

Empty budget: `$$` (to distinguish between 'tasks' and 'statements')

### Sentences

Judgment: `term. :|: %1.0; 0.9%`

Goal: `term! :|: %1.0; 0.9%`

Question: `term? :|:`

Request: `term@ :|:`

Judgment/Single truth value: `term. :|: %1%`

Goal/Single truth value: `term! :|: %1%`

Question/No timestamp: `term?`

Request/No timestamp: `term@`

Judgment/Empty truth value: `term. :|:`

Goal/Empty truth value: `term! :|:`

### Tasks

Complete form: `$0.5; 0.5; 0.5$ term. :!137: %1.0; 0.9%`

Double budget: `$0.5; 0.5$ term.`

Single budget: `$0.5$ term.`

Empty budget: `$$ term.` (An empty budget is also a task, not a statement)
