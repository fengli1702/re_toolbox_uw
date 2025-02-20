import ghidra.app.script.GhidraScript;
import ghidra.program.model.listing.*;
import ghidra.app.decompiler.*;

public class DecompileFunctionH extends GhidraScript {

    @Override
    public void run() throws Exception {
        // Get the function name from the script arguments
        String functionName = getScriptArgs()[0];
        if (functionName == null || functionName.isEmpty()) {
            println("No function name provided.");
            return;
        }

        Function function = findFunctionByName(functionName);
        if (function == null) {
            println("Function '" + functionName + "' not found.");
            return;
        }

        DecompInterface decompiler = new DecompInterface();
	// get default decompiler config
	//decompiler.setOptions(currentProgram.getOptions("Decompiler"));

        decompiler.openProgram(currentProgram);

        DecompileResults results = decompiler.decompileFunction(function, 60, monitor);
        if (results.getDecompiledFunction() != null) {
            String decompiledCode = results.getDecompiledFunction().getC();
            println(decompiledCode);
        } else {
            println("Decompilation failed: " + results.getErrorMessage());
        }

        decompiler.dispose();
    }

    private Function findFunctionByName(String functionName) {
        FunctionManager functionManager = currentProgram.getFunctionManager();
        FunctionIterator functions = functionManager.getFunctions(true);
        while (functions.hasNext()) {
            Function function = functions.next();
            if (function.getName().equals(functionName)) {
                return function;
            }
        }
        return null;
    }
}
