
const invoke = window.__TAURI__.invoke
var counter = document.getElementById('counter')
var incBtn = document.getElementById('inc-btn')
var decBtn = document.getElementById('dec-btn')
//invoke('greet', {name: 'World'}).then((response) => console.log(response))
invoke('get_counter').then((response) => counter.innerText=response)

incBtn.addEventListener('click', function() {
    invoke('inc', {cnt: parseInt(counter.innerText)}).then((response) => counter.innerText=response)
})

decBtn.addEventListener('click', function() {
    invoke('dec', {cnt: parseInt(counter.innerText)}).then((response) => counter.innerText=response)
})