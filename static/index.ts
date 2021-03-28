import { run, Home } from '../Cargo.toml'
import Chart from 'chart.js';

// Start the show
run()

document.getElementById('jsonStart').onclick = function() {
  // For debugging, can remove when not needed:
  // var files = document.getElementById('jsonImport').files;
  // if (files.length <= 0) {return false;}
  // let fr = new FileReader();
  // fr.onload = function(e) { 
  // console.log("File preview/debug loaded via JS");
  // var result = JSON.parse(e.target.result);
  // var formatted = JSON.stringify(result, null, 2);
  //   document.getElementById('result').value = formatted;
  // }
  // fr.readAsText(files.item(0));


const randCols = [
    "#f3a683","#f7d794","#778beb","#e77f67",
    "#cf6a87","#786fa6","#f8a5c2","#63cdda","#ea8685",
    "#596275","#f19066","#f5cd79","#546de5","#e15f41",
    "#c44569","#574b90","#f78fb3","#3dc1d3","#e66767",
    "#303952","#d35400","#c0392b","#bdc3c7","#7f8c8d",
    "#55efc4","#81ecec","#74b9ff","#a29bfe","#dfe6e9",
    "#00b894","#00cec9","#0984e3","#6c5ce7","#ffeaa7",
    "#fab1a0","#ff7675","#fd79a8","#fdcb6e","#e17055",
    "#d63031","#feca57","#5f27cd","#54a0ff","#01a3a4"
]


let ctx_tracks = document.getElementById('tracks').getContext('2d');
let chart_tracks = JSON.parse(document.getElementById("chart_tracks").textContent);

let tracks = new Chart(ctx_tracks, {
      type: 'pie',
      data: {
        labels: Object.keys(chart_tracks),
        datasets: [{
            label: '# of Tracks per file',
            data: Object.values(chart_tracks),
            backgroundColor: randCols,
            borderWidth: 0,
        }]},
        options: { legend: {display: true, position: "bottom", labels: {fontColor: "white", fontStyle: "bold"}}},
      
    });


let ctx_formats = document.getElementById('formats').getContext('2d');
let chart_formats = JSON.parse(document.getElementById("chart_formats").textContent);

let formats = new Chart(ctx_formats, {
      type: 'pie',
      data: {
        labels: Object.keys(chart_formats),
        datasets: [{
            label: 'Formats in collection',
            data: Object.values(chart_formats),
            backgroundColor: randCols,
            borderWidth: 0,
        }]},
        options: { legend: {display: true, position: "bottom", labels: {fontColor: "white", fontStyle: "bold"}}},
      
    });


};


