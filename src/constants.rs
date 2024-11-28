pub fn command_set_to_name(command_set: u8) -> &'static str {
    match command_set {
        1 => "VirtualMachine",
        2 => "ReferenceType",
        3 => "ClassType",
        4 => "ArrayType",
        5 => "InterfaceType",
        6 => "Method",
        8 => "Field",
        9 => "ObjectReference",
        10 => "StringReference",
        11 => "ThreadReference",
        12 => "ThreadGroupReference",
        13 => "ArrayReference",
        14 => "ClassLoaderReference",
        15 => "EventRequest",
        16 => "StackFrame",
        17 => "ClassObjectReference",
        18 => "ModuleReference",
        64 => "Event",
        _ => "Unknown",
    }
}

pub fn command_to_name(command_set: u8, command: u8) -> &'static str {
    match command_set {
        1 => match command {
            1 => "Version",
            2 => "ClassesBySignature",
            3 => "AllClasses",
            4 => "AllThreads",
            5 => "TopLevelThreadGroups",
            6 => "Dispose",
            7 => "IDSizes",
            8 => "Suspend",
            9 => "Resume",
            10 => "Exit",
            11 => "CreateString",
            12 => "Capabilities",
            13 => "ClassPaths",
            14 => "DisposeObjects",
            15 => "HoldEvents",
            16 => "ReleaseEvents",
            17 => "CapabilitiesNew",
            18 => "RedefineClasses",
            19 => "SetDefaultStratum",
            20 => "AllClassesWithGeneric",
            21 => "InstanceCounts",
            22 => "AllModules",
            _ => "Unknown",
        },
        2 => match command {
            1 => "Signature",
            2 => "ClassLoader",
            3 => "Modifiers",
            4 => "Fields",
            5 => "Methods",
            6 => "GetValues",
            7 => "SourceFile",
            8 => "NestedTypes",
            9 => "Status",
            10 => "Interfaces",
            11 => "ClassObject",
            12 => "SourceDebugExtension",
            13 => "SignatureWithGeneric",
            14 => "FieldsWithGeneric",
            15 => "MethodsWithGeneric",
            16 => "Instances",
            17 => "ClassFileVersion",
            18 => "ConstantPool",
            19 => "Module",
            _ => "Unknown",
        },
        3 => match command {
            1 => "Superclass",
            2 => "SetValues",
            3 => "InvokeMethod",
            4 => "NewInstance",
            _ => "Unknown",
        },
        4 => match command {
            1 => "NewInstance",
            _ => "Unknown",
        },
        5 => match command {
            1 => "InvokeMethod",
            _ => "Unknown",
        },
        6 => match command {
            1 => "LineTable",
            2 => "VariableTable",
            3 => "Bytecodes",
            4 => "IsObsolete",
            5 => "VariableTableWithGeneric",
            _ => "Unknown",
        },
        9 => match command {
            1 => "ReferenceType",
            2 => "GetValues",
            3 => "SetValues",
            5 => "MonitorInfo",
            6 => "InvokeMethod",
            7 => "DisableCollection",
            8 => "EnableCollection",
            9 => "IsCollected",
            10 => "ReferringObjects",
            _ => "Unknown",
        },
        10 => match command {
            1 => "Value",
            _ => "Unknown",
        },
        11 => match command {
            1 => "Name",
            2 => "Suspend",
            3 => "Resume",
            4 => "Status",
            5 => "ThreadGroup",
            6 => "Frames",
            7 => "FrameCount",
            8 => "OwnedMonitors",
            9 => "CurrentContendedMonitor",
            10 => "Stop",
            11 => "Interrupt",
            12 => "SuspendCount",
            13 => "OwnedMonitorsStackDepthInfo",
            14 => "ForceEarlyReturn",
            15 => "IsVirtual",
            _ => "Unknown",
        },
        12 => match command {
            1 => "Name",
            2 => "Parent",
            3 => "Child",
            _ => "Unknown",
        },
        13 => match command {
            1 => "Length",
            2 => "GetValues",
            3 => "SetValues",
            _ => "Unknown",
        },
        14 => match command {
            1 => "VisibleClasses",
            _ => "Unknown",
        },
        15 => match command {
            1 => "Set",
            2 => "Clear",
            3 => "ClearAllBreakpoints",
            _ => "Unknown",
        },
        16 => match command {
            1 => "GetValues",
            2 => "SetValues",
            3 => "ThisObject",
            4 => "PopFrames",
            _ => "Unknown",
        },
        17 => match command {
            1 => "ReflectedType",
            _ => "Unknown",
        },
        18 => match command {
            1 => "Name",
            2 => "ClassLoader",
            _ => "Unknown",
        },
        64 => match command {
            100 => "Composite",
            _ => "Unknown",
        },
        _ => "Unknown",
    }
}

pub fn header_to_string(header: &[u8; 11]) -> String {
    let len = u32::from_be_bytes(header[..4].try_into().unwrap());
    let id = u32::from_be_bytes(header[4..8].try_into().unwrap());

    if header[8] == 0x80 {
        let error = u16::from_be_bytes(header[9..11].try_into().unwrap());
        format!("length = {:<5}| id = {:<4}| error = {}", len, id, error)
    } else {
        let command_set = header[9];
        let command = header[10];

        format!(
            "length = {:<5}| id = {:<4}| command = {}.{}",
            len,
            id,
            command_set_to_name(command_set),
            command_to_name(command_set, command)
        )
    }
}
