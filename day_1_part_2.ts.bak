import * as fs from 'fs';

type State={
    window:number[]
    count:number
}
const data = fs.readFileSync('./data/day_1_1.dat', 'utf-8');

// split the contents by new line
const initState:State={count:0,window:[]}
const count = data.split(/\r?\n/).map(line=>parseInt(line)).reduce(
    (state:State,val)=>{
    const newWindow = state.window.slice(-2).concat(val)
    if(state.window.length !== 3){
        return {window:newWindow,count:state.count}
    }
    const lastWindowCount = state.window.reduce((old,val)=>old+val)
    const newWindowCount = newWindow.reduce((old,val)=>val+old)
    return {window:newWindow,count:state.count+(lastWindowCount<newWindowCount?1:0)}
    
},initState).count
console.log("Count "+count)