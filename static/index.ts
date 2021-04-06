import { run } from '../Cargo.toml'
import Chart from 'chart.js';

// Start the show
run()

const opts = { responsive: true, aspectRatio: 1.33, legend: {display: true, maxHeight: 500, position: "bottom", labels: {fontColor: "white", fontStyle: "bold"}}};
const bar_opts = { responsive: true, aspectRatio: 1.33, legend: {display: false }, scales: {xAxes: [{ticks: {fontColor: "white",fontStyle: "bold"}}]}};

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

let tracks, formats, color_spaces, audio_codecs, video_codecs, audio_bitdepths, 
    video_bitdepths, video_standards, chroma_subsamplings, file_extensions, dimensions;

document.getElementById('jsonStart').onclick = function() {

  if (tracks) {
    tracks.destroy();
    formats.destroy();
    color_spaces.destroy();
    audio_codecs.destroy();
    video_codecs.destroy();
    audio_bitdepths.destroy();
    video_bitdepths.destroy();
    video_standards.destroy();
    chroma_subsamplings.destroy();
    file_extensions.destroy();
    dimensions.destroy();
  }

  // tracks 
  let chart_tracks = JSON.parse(document.getElementById("chart_tracks").textContent);
  tracks = new Chart(document.getElementById('tracks').getContext('2d'), {
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
  let chart_formats = JSON.parse(document.getElementById("chart_formats").textContent);
  formats = new Chart(document.getElementById('formats').getContext('2d'), {
        type: 'pie',
        data: {
          labels: Object.keys(chart_formats),
          datasets: [{
              label: 'Formats',
              data: Object.values(chart_formats),
              backgroundColor: randCols,
              borderWidth: 0,
          }]},
          options: opts,     
      });

  // color_spaces
  let chart_color_spaces = JSON.parse(document.getElementById("chart_color_spaces").textContent);
  color_spaces = new Chart(document.getElementById('color_spaces').getContext('2d'), {
        type: 'pie',
        data: {
          labels: Object.keys(chart_color_spaces),
          datasets: [{
              label: 'Colorspaces',
              data: Object.values(chart_color_spaces),
              backgroundColor: randCols,
              borderWidth: 0,
          }]},
          options: opts,     
      });

  // dimensions
  let chart_dimensions = JSON.parse(document.getElementById("chart_dimensions").textContent);
  dimensions = new Chart(document.getElementById('dimensions').getContext('2d'), {
        type: 'pie',
        data: {
          labels: Object.keys(chart_dimensions),
          datasets: [{
              label: 'Dimensions',
              data: Object.values(chart_dimensions),
              backgroundColor: randCols,
              borderWidth: 0,
          }]},
          options: opts,
      });

  // audio_codecs
  let chart_audio_codecs = JSON.parse(document.getElementById("chart_audio_codecs").textContent);
  audio_codecs = new Chart(document.getElementById('audio_codecs').getContext('2d'), {
        type: 'pie',
        data: {
          labels: Object.keys(chart_audio_codecs),
          datasets: [{
              label: 'Audio codecs',
              data: Object.values(chart_audio_codecs),
              backgroundColor: randCols,
              borderWidth: 0,
          }]},
          options: opts,     
      });


  // video_codecs
  let chart_video_codecs = JSON.parse(document.getElementById("chart_video_codecs").textContent);
  video_codecs = new Chart(document.getElementById('video_codecs').getContext('2d'), {
        type: 'pie',
        data: {
          labels: Object.keys(chart_video_codecs),
          datasets: [{
              label: 'Video codecs',
              data: Object.values(chart_video_codecs),
              backgroundColor: randCols,
              borderWidth: 0,
          }]},
          options: opts,
      });


  // audio_bitdepths
  let chart_audio_bitdepths = JSON.parse(document.getElementById("chart_audio_bitdepths").textContent);
  audio_bitdepths = new Chart(document.getElementById('audio_bitdepths').getContext('2d'), {
        type: 'pie',
        data: {
          labels: Object.keys(chart_audio_bitdepths),
          datasets: [{
              label: 'Audio bit depths',
              data: Object.values(chart_audio_bitdepths),
              backgroundColor: randCols,
              borderWidth: 0,
          }]},
          options: opts,     
      });


  // video_bitdepths
  let chart_video_bitdepths = JSON.parse(document.getElementById("chart_video_bitdepths").textContent);
  video_bitdepths = new Chart(document.getElementById('video_bitdepths').getContext('2d'), {
        type: 'pie',
        data: {
          labels: Object.keys(chart_video_bitdepths),
          datasets: [{
              label: 'Color depths',
              data: Object.values(chart_video_bitdepths),
              backgroundColor: randCols,
              borderWidth: 0,
          }]},
          options: opts,
      });

  // video_standards
  let chart_video_standards = JSON.parse(document.getElementById("chart_video_standards").textContent);
  video_standards = new Chart(document.getElementById('video_standards').getContext('2d'), {
        type: 'bar',
        data: {
          labels: Object.keys(chart_video_standards),
          datasets: [{
              label: 'Color depths',
              data: Object.values(chart_video_standards),
              backgroundColor: randCols,
              borderWidth: 0,
          }]},
          options: bar_opts,
      });


  // chroma_subsamplings
  let chart_chroma_subsamplings = JSON.parse(document.getElementById("chart_chroma_subsamplings").textContent);
  chroma_subsamplings = new Chart(document.getElementById('chroma_subsamplings').getContext('2d'), {
        type: 'pie',
        data: {
          labels: Object.keys(chart_chroma_subsamplings),
          datasets: [{
              label: 'Chroma subsampling',
              data: Object.values(chart_chroma_subsamplings),
              backgroundColor: randCols,
              borderWidth: 0,
          }]},
          options: opts,
      });

  // file_extensions
  let chart_file_extensions = JSON.parse(document.getElementById("chart_file_extensions").textContent);
  file_extensions = new Chart(document.getElementById('file_extensions').getContext('2d'), {
        type: 'pie',
        data: {
          labels: Object.keys(chart_file_extensions),
          datasets: [{
              label: 'File extensions',
              data: Object.values(chart_file_extensions),
              backgroundColor: randCols,
              borderWidth: 0,
          }]},
          options: opts,
      });

};


