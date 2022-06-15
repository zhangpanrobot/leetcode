let list1 = [6, 5, 8, 1, 4]
let list2 = [5, 8, 3, 6, 9]
let list3 = [2, 6, 1, 7, 10]

function sort(list) {
  let listLen = list.length
  for (let i = 0; i < listLen - 1; i++) {
    for (let j = 0; j < listLen - i - 1; j++) {
      if (list[j] > list[j + 1]) {
        swap(list, j, j + 1)
      }
    }
  }
}

sort(list1)
sort(list2)
sort(list3)
console.log(list1, 'list1')
console.log(list2, 'list2')
console.log(list3, 'list3')

function swap(list, i, j) {
  let temp = list[i]
  list[i] = list[j]
  list[j] = temp
}