```
GENERIC: Converting generic span: Span { start: 10, len: 20, _p: PhantomData<main_38f3a6ec413f5aecb4499655::Raw> }
Result with CliFormat: Span { start: 10, len: 20, _p: PhantomData<main_38f3a6ec413f5aecb4499655::Cooked> }
GENERIC: Converting generic span: Span { start: 30, len: 40, _p: PhantomData<main_38f3a6ec413f5aecb4499655::Cooked> }
Result with JsonFormat: Span { start: 30, len: 40, _p: PhantomData<main_38f3a6ec413f5aecb4499655::Cooked> }
SPECIALIZED: Using already cooked span: Span { start: 50, len: 60, _p: PhantomData<main_38f3a6ec413f5aecb4499655::Cooked> }
Direct cooked result: Span { start: 50, len: 60, _p: PhantomData<main_38f3a6ec413f5aecb4499655::Cooked> }
SPECIALIZED: Converting raw span: Span { start: 70, len: 80, _p: PhantomData<main_38f3a6ec413f5aecb4499655::Raw> }
Direct raw result: Span { start: 70, len: 80, _p: PhantomData<main_38f3a6ec413f5aecb4499655::Cooked> }
```
