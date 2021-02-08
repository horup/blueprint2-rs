import('./pkg').catch(e=>{
    if (e.message != "Using exceptions for control flow, don't mind me. This isn't actually an error!") {
        console.error(e.message);
    }
});