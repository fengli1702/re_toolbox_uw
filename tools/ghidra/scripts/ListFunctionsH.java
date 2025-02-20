import ghidra.app.script.GhidraScript;
import ghidra.program.model.listing.Function;
import ghidra.program.model.listing.FunctionIterator;
import ghidra.program.model.symbol.Symbol;

public class ListFunctionsH extends GhidraScript {
    @Override
    public void run() throws Exception {
        FunctionIterator functions = currentProgram.getFunctionManager().getFunctions(true);
        for (Function function : functions) {
            Symbol symbol = function.getSymbol();
            String mangledName = function.getName();
            String demangledName = symbol.getName(true); // Get the demangled name if available

            if (!mangledName.equals(demangledName)) {
                println(mangledName + " [" + demangledName + "]");
            } else {
                println(mangledName);
            }
        }
    }
}
