FROM gcr.io/distroless/static:nonroot
COPY --chown=nonroot:nonroot ./scheduler /app/
EXPOSE 8080
ENTRYPOINT ["/app/scheduler"]