# DEVELOPER GUIDE

## Defining Nodes

- Each node is defined in its own YAML file.
- YAML files contain data about a node.
- Each YAML file also expresses a node's relationship to its neighbors
  in a tree.
- A neighbor may be a predecessor or a successor. The presentation of
  the text in a YAML file depends on (is a dependency of) the text in
  the YAML files declared as predecessors (successors). That is, a
  predecessor may not appear after a successor in the text, and vice
  versa.
- Certain relationships may be reversed based on user preference. For
  example,
  - If a node represents a generalization of several other nodes, and
    the reader prefers to see the general form of an equation prior to
    exploring the use of that equation under certain assumptions, then
    nodes declared as special cases will be treated as successors and
    nodes treated as generalizations will be treated as predecessors.
    The default is that generalizations will be treated as successors
    and special cases will be treated as predecessors.
