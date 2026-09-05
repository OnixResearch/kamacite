type U24Tree:
  Leaf { value: u24 }
  Node { ~left: U24Tree, ~right: U24Tree }

def sum(tree: U24Tree) -> u24:
  fold tree:
    case U24Tree/Leaf:
      return tree.value
    case U24Tree/Node:
      return tree.left + tree.right

def main() -> u24:
  tree = U24Tree/Node {
    left: U24Tree/Node {
      left: U24Tree/Leaf { value: 16777215 },
      right: U24Tree/Leaf { value: 1 }
    },
    right: U24Tree/Node {
      left: U24Tree/Leaf { value: 0 },
      right: U24Tree/Leaf { value: 1 }
    }
  }
  return sum(tree)
