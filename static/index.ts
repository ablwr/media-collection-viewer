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

let ctx = document.getElementById('myChart').getContext('2d');
let chart_tracks = JSON.parse(document.getElementById("chart_tracks").textContent);

var myChart = new Chart(ctx, {
      type: 'pie',
      data: {
        labels: Object.keys(chart_tracks),
        datasets: [{
            label: '# of Tracks per file',
            data: Object.values(chart_tracks),
            backgroundColor: [
                'rgba(255, 99, 132, 1)',
                'rgba(54, 162, 235, 1)',
                'rgba(255, 206, 86, 1)',
                'rgba(255, 230, 255, 1)',
            ],
            borderWidth: 0,
        }]},
        options: { legend: {display: true, position: "bottom", labels: {fontColor: "white", fontStyle: "bold"}}},
      
    });



};


