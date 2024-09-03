import test from 'ava'

import { windSum, linuxSum, winHideTaskBar,  linuxHideTaskBar } from '../index.js'

test('windows sum from native', (t) => {
  t.is(windSum(1, 2), 3)
})


test('linux sum from native', (t) => {
  t.is(linuxSum(1, 2), 3)
})


test('windows hide from native', (t) => {
  t.is(winHideTaskBar(1), '隐藏任务栏')
})

test('linux hide from native', (t) => {
  t.is(linuxHideTaskBar(1), '指令错误')
})


