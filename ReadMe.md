# JDeduplix: AI-Powered Smart Deduplication System

JDeduplix is a cutting-edge deduplication system that leverages artificial intelligence to provide intelligent, accurate, and efficient data deduplication across multiple data types.

## 📊 System Architecture

### Full Stack Overview

```mermaid
%%{init: {'theme': 'dark', 'themeVariables': { 'darkMode': true }}}%%
flowchart TD
    classDef frontend fill:#42b883,stroke:#35495e,stroke-width:2px,color:#1a1c1d
    classDef backend fill:#2b7489,stroke:#1a1c1d,stroke-width:2px,color:white
    classDef storage fill:#ff6b6b,stroke:#c92a2a,stroke-width:2px,color:white
    classDef api fill:#ff9f43,stroke:#e67e22,stroke-width:2px,color:#1a1c1d
    classDef dist fill:#a55eea,stroke:#8854d0,stroke-width:2px,color:white
    classDef p2p fill:#00b894,stroke:#00cec9,stroke-width:2px,color:#1a1c1d
    classDef adapter fill:#6c5ce7,stroke:#a29bfe,stroke-width:2px,color:white

    subgraph FE[Frontend Layer]
        direction TB
        UI[Tauri + Vue UI]:::frontend
        VIS[Visualizations]:::frontend
        EXP[Export Tools]:::frontend
        APIC[API Console]:::frontend
        P2PC[P2P Controls]:::frontend
    end

    subgraph BE[Backend Layer]
        direction TB
        DE[Deduplication Engine]:::backend
        AI[AI Processing]:::backend
        IDX[Smart Indexing]:::backend
        DIST[Distributed Coordinator]:::dist
    end

    subgraph ST[Storage Layer]
        direction TB
        DB[(Databases)]:::storage
        FS[(File System)]:::storage
        VDB[(Vector DBs)]:::storage
        GIT[(Version Control)]:::storage
    end

    subgraph AG[API Gateway]
        direction TB
        REST[REST API]:::api
        WS[WebSocket API]:::api
        SDK[Client SDKs]:::api
    end

    subgraph P2P[P2P Network Layer]
        direction TB
        
        subgraph P2PA[P2P Adapter Interface]
            direction TB
            PCORE[P2P Core]:::p2p
            PADAPT[Protocol Adapter]:::adapter
        end

        subgraph PROT[Protocol Implementations]
            direction TB
            LP2P[LibP2P Stack]:::p2p
            HOLE[Holepunch Stack]:::p2p
        end

        subgraph SERV[P2P Services]
            direction TB
            DHT[DHT Routing]:::p2p
            IPFS[IPFS Storage]:::p2p
            HYPER[Hypercore]:::p2p
            DISCO[Peer Discovery]:::p2p
        end
    end

    UI --> DE
    DE --> AI
    AI --> IDX
    IDX --> DB & FS & VDB & GIT
    IDX --> UI
    UI --> VIS & EXP & APIC & P2PC
    
    DE <--> DIST
    
    APIC & UI --> REST & WS
    REST & WS --> SDK

    %% P2P Connections
    P2PC --> PCORE
    PCORE --> PADAPT
    PADAPT --> LP2P & HOLE
    LP2P --> DHT & IPFS
    HOLE --> HYPER & DISCO
    DE <--> PCORE
    IDX --> IPFS & HYPER

    style FE fill:#1a1c1d,stroke:#42b883,stroke-width:2px
    style BE fill:#1a1c1d,stroke:#2b7489,stroke-width:2px
    style ST fill:#1a1c1d,stroke:#ff6b6b,stroke-width:2px
    style AG fill:#1a1c1d,stroke:#ff9f43,stroke-width:2px
    style P2P fill:#1a1c1d,stroke:#00b894,stroke-width:2px
    style P2PA fill:#1a1c1d,stroke:#6c5ce7,stroke-width:2px
    style PROT fill:#1a1c1d,stroke:#00b894,stroke-width:2px
    style SERV fill:#1a1c1d,stroke:#00b894,stroke-width:2px
```

### Backend Architecture

```mermaid
%%{init: {'theme': 'dark', 'themeVariables': { 'darkMode': true }}}%%
flowchart TD
    classDef engine fill:#2b7489,stroke:#1a1c1d,stroke-width:2px,color:white
    classDef ai fill:#6b9fff,stroke:#2d5a9e,stroke-width:2px,color:#1a1c1d
    classDef storage fill:#ff6b6b,stroke:#c92a2a,stroke-width:2px,color:white
    classDef dist fill:#a55eea,stroke:#8854d0,stroke-width:2px,color:white
    classDef version fill:#20bf6b,stroke:#26de81,stroke-width:2px,color:#1a1c1d
    classDef p2p fill:#00b894,stroke:#00cec9,stroke-width:2px,color:#1a1c1d
    classDef adapter fill:#6c5ce7,stroke:#a29bfe,stroke-width:2px,color:white

    subgraph Core[Deduplication Core]
        direction TB
        A[Smart Classifier]:::engine
        B[Deduplication Engine]:::engine
        C[AI Processing Pipeline]:::ai
        D1[ML Text Deduper]:::ai
        D2[GNN JSON Deduper]:::ai
        D3[CNN Image Deduper]:::ai
        D4[Binary Deduper]:::ai
        E[ML Conflict Resolver]:::ai
        F[Auto-Tuning]:::ai
        G[Vector Indexing]:::engine
        H1[FAISS HNSW]:::engine
        H2[Perceptual Hash]:::engine
        H3[Graph Matching]:::engine
        I[Storage Layer]:::storage
        J1[Sled DB]:::storage
        J2[File System]:::storage
        J3[Vector DB]:::storage
        
        %% Core Components
        K[Model Registry]:::ai
        L[Feedback Loop]:::ai
        M[Version Control]:::version
        N[Distributed Coordinator]:::dist
        O[Worker Nodes]:::dist
        P[API Gateway]:::engine
        
        %% P2P Components
        Q[P2P Manager]:::p2p
        R[Protocol Adapter]:::adapter
        S1[LibP2P Implementation]:::p2p
        S2[Holepunch Implementation]:::p2p
        T[Peer Discovery]:::p2p
        U[Content Router]:::p2p
        V[Replication Manager]:::p2p
        W[Storage Adapter]:::adapter
        X1[IPFS Storage]:::p2p
        X2[Hypercore Storage]:::p2p
        
        A --> B
        B --> C
        C --> D1 & D2 & D3 & D4
        D1 & D2 & D3 & D4 --> E
        E --> F
        B --> G
        G --> H1 & H2 & H3
        H1 & H2 & H3 --> I
        I --> J1 & J2 & J3
        
        %% Core Connections
        E --> L
        L --> K
        K --> C
        B --> M
        N --> O
        O --> B
        B --> P
        
        %% P2P Connections
        B <--> Q
        Q --> R
        R --> S1 & S2
        S1 & S2 --> T
        T --> U
        U --> V
        V --> W
        W --> X1 & X2
        X1 & X2 --> I
        
        %% Feedback Loops
        L --> |Model Updates| D1 & D2 & D3 & D4
        F --> |Threshold Updates| L
        M --> |Version History| I
        V --> |P2P Updates| L
    end

    style Core fill:#1a1c1d,stroke:#2b7489,stroke-width:2px
```

## 🌟 Key Features

- **Smart Classification**: AI-powered detection and routing of different data types
- **Multi-Modal Deduplication**: Specialized engines for text, JSON, images, and binary data
- **ML-Enhanced Matching**: Combines exact and fuzzy matching using advanced AI models
- **Intelligent Conflict Resolution**: Machine learning-assisted conflict handling with human oversight
- **High-Performance Search**: Vector-based similarity search using FAISS HNSW
- **Flexible Storage**: Pluggable storage backends supporting Sled DB, file system, and vector databases
- **P2P Capabilities**: Decentralized data sharing and deduplication using IPFS and LibP2P

## 🏗️ Architecture

### Frontend (Tauri + Vue)
- Modern, responsive UI for data visualization and management
- AI confidence score displays and similarity heatmaps
- Interactive conflict resolution interface
- Export and download capabilities
- P2P network controls and protocol selection

### Backend (Rust + AI)
- **Deduplication Engine**:
  - ML Text Deduper
  - GNN JSON Deduper
  - CNN Image Deduper
  - Deep Learning Binary Deduper
  
- **Smart Indexing**:
  - FAISS HNSW Vector Search
  - Perceptual Hashing
  - Graph Matching Algorithms

- **ML Conflict Resolution**:
  - Confidence-weighted decision making
  - Automated threshold tuning
  - Human-in-the-loop review system

- **P2P Network Layer**:
  - Pluggable protocol architecture
  - LibP2P implementation with IPFS storage
  - Holepunch implementation with Hypercore storage
  - Protocol-agnostic adapter interface
  - Unified peer discovery and routing
  - Cross-protocol replication support

### Storage Options
- Sled DB for high-performance local storage
- File system integration for simple deployments
- Vector DB support (Weaviate/Pinecone)
- IPFS for LibP2P-based storage
- Hypercore for Holepunch-based storage

## 🚀 Getting Started

[Coming Soon]

## 🛠️ Development

[Coming Soon]

## 📚 Documentation

[Coming Soon]

## 🤝 Contributing

[Coming Soon]

## 📄 License

[Coming Soon]

## 🔗 Links

[Coming Soon]