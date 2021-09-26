let list1 = [6, 5, 8, 1, 4]
let list2 = [5, 8, 3, 6, 9]
let list3 = [2, 6, 1, 7, 10]

function sort(list) {
  let mid = list.length >> 1
  let left = list.slice(0, mid)
  let right = list.slice(mid)
  if (left.length > 1) {
    left = sort(left)
  }
  if (right.length > 1) {
    right = sort(right)
  }
  return merge(left, right)
}

function merge(left, right) {
  if (!left || !right) {
    return left || right
  }
  let arr = []
  let leftLen = left.length
  let rightLen = right.length
  let i = 0;
  let j = 0;
  for (let k = 0; k < leftLen + rightLen - 1; k++) {
    if (left[i] < right[j]) {
      arr.push(left[i])
      i++
    } else {
      arr.push(right[j])
      j++
    }
  }
  if (i < leftLen) {
    arr = arr.concat(left.slice(i))
  }
  if (j < rightLen) {
    arr = arr.concat(right.slice(j))
  }
  return arr
}

list1 = sort(list1)
list2 = sort(list2)
list3 = sort(list3)
console.log(list1, 'list1')
console.log(list2, 'list2')
console.log(list3, 'list3')

function swap(list, i, j) {
  let temp = list[i]
  list[i] = list[j]
  list[j] = temp
}