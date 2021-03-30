import { run } from '../Cargo.toml'
import Chart from 'chart.js';

// Start the show
run()

const opts = { legend: {display: true, position: "bottom", labels: {fontColor: "white", fontStyle: "bold"}}};
const randCols = [
    "#25CCF7","#FD7272","#54a0ff","#00d2d3",
    "#1abc9c","#2ecc71","#3498db","#9b59b6","#34495e",
    "#16a085","#27ae60","#2980b9","#8e44ad","#2c3e50",
    "#f1c40f","#e67e22","#e74c3c","#ecf0f1","#95a5a6",
    "#f39c12","#d35400","#c0392b","#bdc3c7","#7f8c8d",
    "#55efc4","#81ecec","#74b9ff","#a29bfe","#dfe6e9",
    "#00b894","#00cec9","#0984e3","#6c5ce7","#ffeaa7",
    "#fab1a0","#ff7675","#fd79a8","#fdcb6e","#e17055",
    "#d63031","#feca57","#5f27cd","#54a0ff","#01a3a4"
]

document.getElementById('jsonStart').onclick = function() {

  // tracks 
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
          options: opts,    
      });

  // formats
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
          options: opts,     
      });

  // color_spaces
  let ctx_color_spaces = document.getElementById('color_spaces').getContext('2d');
  let chart_color_spaces = JSON.parse(document.getElementById("chart_color_spaces").textContent);

  let color_spaces = new Chart(ctx_color_spaces, {
        type: 'pie',
        data: {
          labels: Object.keys(chart_color_spaces),
          datasets: [{
              label: 'color_spaces in collection',
              data: Object.values(chart_color_spaces),
              backgroundColor: randCols,
              borderWidth: 0,
          }]},
          options: opts,     
      });

  // audio_codecs
  let ctx_audio_codecs = document.getElementById('audio_codecs').getContext('2d');
  let chart_audio_codecs = JSON.parse(document.getElementById("chart_audio_codecs").textContent);

  let audio_codecs = new Chart(ctx_audio_codecs, {
        type: 'pie',
        data: {
          labels: Object.keys(chart_audio_codecs),
          datasets: [{
              label: 'audio_codecs in collection',
              data: Object.values(chart_audio_codecs),
              backgroundColor: randCols,
              borderWidth: 0,
          }]},
          options: opts,     
      });


  // video_codecs
  let ctx_video_codecs = document.getElementById('video_codecs').getContext('2d');
  let chart_video_codecs = JSON.parse(document.getElementById("chart_video_codecs").textContent);

  let video_codecs = new Chart(ctx_video_codecs, {
        type: 'pie',
        data: {
          labels: Object.keys(chart_video_codecs),
          datasets: [{
              label: 'video_codecs in collection',
              data: Object.values(chart_video_codecs),
              backgroundColor: randCols,
              borderWidth: 0,
          }]},
          options: opts,
       
      });


  // audio_bitdepths
  let ctx_audio_bitdepths = document.getElementById('audio_bitdepths').getContext('2d');
  let chart_audio_bitdepths = JSON.parse(document.getElementById("chart_audio_bitdepths").textContent);

  let audio_bitdepths = new Chart(ctx_audio_bitdepths, {
        type: 'pie',
        data: {
          labels: Object.keys(chart_audio_bitdepths),
          datasets: [{
              label: 'audio_bitdepths in collection',
              data: Object.values(chart_audio_bitdepths),
              backgroundColor: randCols,
              borderWidth: 0,
          }]},
          options: opts,     
      });


  // video_bitdepths
  let ctx_video_bitdepths = document.getElementById('video_bitdepths').getContext('2d');
  let chart_video_bitdepths = JSON.parse(document.getElementById("chart_video_bitdepths").textContent);

  let video_bitdepths = new Chart(ctx_video_bitdepths, {
        type: 'pie',
        data: {
          labels: Object.keys(chart_video_bitdepths),
          datasets: [{
              label: 'video_bitdepths in collection',
              data: Object.values(chart_video_bitdepths),
              backgroundColor: randCols,
              borderWidth: 0,
          }]},
          options: opts,
       
      });

};


