import { HttpClient, HttpErrorResponse } from '@angular/common/http';
import { Component } from '@angular/core';
//google-chrome  --user-data-dir=”/var/tmp/Chrome” --disable-web-security
//chromium-browser  --user-data-dir=”/var/tmp/Chrome” --disable-web-security

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent {
  title = 'grid'

  y = 28
  x = 28
  grid: number[][] = Array(this.y).fill(0).map(() => Array(this.x).fill(0))

  loading: boolean = false
  error: string = ""
  numbi: number[] = []
  winner: number = -1


  boop_x: number = 0 
  boop_y: number = 0 
  boop_w: number = 0 
  boop_z: number = 0 
  constructor(public digitClient: HttpClient) {
    setInterval(() => {
      let boops = this.grid.map(row => row.map(pix => {
        if (pix > 230) { return 255 }
        if (pix > 150) { return 165 }
        if (pix > 100) { return 15  }
        return 0;
      }))
      this.submitBoops(boops)
    }, 10)
  }

  boop(i: number, j: number) {
    this.grid[i][j] += 60

    this.grid[i][j-1] += 24
    this.grid[i][j+1] += 24
    this.grid[i-1][j] += 24
    this.grid[i+1][j] += 24
    
    this.grid[i-1][j-1] += 7
    this.grid[i+1][j+1] += 7
    this.grid[i-1][j+1] += 7
    this.grid[i+1][j-1] += 7
  }

  clearBoops() {
    this.grid = Array(28).fill(0).map(() => Array(28).fill(0));
  }

  submitBoops(boops: (0 | 255 | 165 | 15)[][]) {
    if (!boops.flat().some(pix => pix > 0))
      return

    this.error = ""
    //this.loading = true
    this.digitClient.get<result>("http://localhost:6969/ai/" + boops.flat().join(",")).subscribe({
      next: (data) => {
        this.numbi = data.result
        this.winner = data.result.indexOf(Math.max(...data.result));
        this.loading = false
      },
      error: (e: HttpErrorResponse) => {
        if (e.status == 0) this.error = "Server is down"
        else this.error = e.error
        
        this.loading = false
      },
    })
  }

  digitStyle(percentage: number, winner: boolean): string {
    let barColor = winner ? "aquamarine" : "white"
    let p = Math.round(percentage*100)

    return "linear-gradient(90deg, "+barColor+" "+p+"%, rgba(0,0,0,0) "+p+"%)";
  }
}

interface result {
  result: number[]
}

