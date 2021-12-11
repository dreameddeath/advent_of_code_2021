import * as fs from 'fs';

type State={
    last?:number;
    count:number
}
const data = fs.readFileSync('./data/day_1_1.dat', 'utf-8');

// split the contents by new line
const initState:State={count:0}
const count = data.split(/\r?\n/).map(line=>parseInt(line)).reduce(
    (state,val)=>{
    if(state.last!==undefined && val>state.last){
        return {last:val,count:state.count+1}
    }
    else{
        return {last:val,count:state.count}
    }
},initState).count
console.log("Count "+count)