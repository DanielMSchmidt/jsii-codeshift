# JSII-Codeshift

The goal of this project is to build a simplified tool similar to [jscodeshift](https://github.com/facebook/jscodeshift) that allows users to script refactorings, but across all languages supported by JSII. To achieve this we aim to limit the scope of the API to the subset of features we think are relevant for CDK projects (where JSII is normally used).

## TODOs

- [ ] Typescript
  - [x] Import Statements
  - [ ] Class Definition
  - [ ] Class Instanciation
  - [ ] Variable Assignments
  - [ ] Method Calls
- [ ] Python
- [ ] Java
- [ ] C#
- [ ] Go

## Usage

```
   ┌───────────────────────────────────┐  jsii-codeshift
┌──│ TS / Python / Java / C# / Go Code │◀─────┐
│  └───────────────────────────────────┘      │
│                                             │
│                                             │
│                                ┌─────────────────────────┐
│                                │  You changing the AST   │
│                                └─────────────────────────┘
│                                             ▲
│jsii-codeshift  ┌─────────────────┐          │
└───────────────▶│ Simplified AST  │──────────┘
                 └─────────────────┘
```
