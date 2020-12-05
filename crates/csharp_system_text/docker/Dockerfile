FROM mcr.microsoft.com/dotnet/sdk:3.1

ARG MAIN

WORKDIR /work
COPY /csharp.csproj /work/csharp.csproj
COPY /Program.cs /work/Program.cs

COPY /gen /work/
RUN sed -i -e "s/MAIN/$MAIN/g" /work/Program.cs

RUN dotnet build
ENTRYPOINT dotnet run
